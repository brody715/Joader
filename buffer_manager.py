import signal, os, sys
sys.path.append("/home/xj/proj/DM/pytorch-imagenet/Loader/")

import multiprocessing
import queue
import threading
from mylog import *
import mmap
from loader import Loader
from replacer import Replacer, RReplacer
from concurrent.futures import ThreadPoolExecutor
from monitor import *
from buffer import Buffer

class BufferManger(object):
    def __init__(self, name, data_len, cap=16, create=False, size=0):
        #buffer
        if create:
            logging.info("create buffer with size=%d and datalen=%d", size, data_len)
        self.buffer = Buffer(name, data_len, create, size)

        # start loader service
        self.id_queue = multiprocessing.Manager().Queue(maxsize=cap)
        self.data_queue = multiprocessing.Manager().Queue(maxsize=cap)
        self.loader = multiprocessing.Process(
            target=Loader.loading, args=(self.id_queue, self.data_queue, name, int(cap/2)))
        self.loader.start()
        assert(self.loader.is_alive() == True)

        # table
        self.data_lock = threading.Lock()
        self.id_table = {}   #id -> datanode
        self.data_refs = {}  #id -> refs
        self.task_tails = {}
        self.task_heads = {}

        self.pending_id_lock = threading.Lock()
        self.pending_id = {} #id->namelist

        #replacer
        self.replacer = Replacer()
        # self.replacer = RReplacer()

        #monitor
        self.hit = 0
        self.miss = 0
        # start a thread to listen data queue
        pool = ThreadPoolExecutor(max_workers=1)
        pool.submit(self.listener)
        # t = threading.Thread(target=self.listener(), args=())
        # t.start()
        
    def listener(self):
        # logging.info("listener listen loader")
        while True:
            # try:
            item = self.data_queue.get()
            #     p_ticker = m.tiker("data process")
            #     p_ticker.end()
            # p_ticker.print_avg(128, 128)
            # except:
            #     logging.error("listener read data queue error")
            #     exit(0)
            # w_ticker = m.tiker("data write")
            # w_ticker.start()
            data_id, data_idx = item
            with self.pending_id_lock:
                name_list, expect_diff = self.pending_id[data_id]
                del self.pending_id[data_id]
            with self.data_lock:
                self.id_table[data_id] = data_idx
            logging.info("listener get data %d in %d with tasks %s", data_id, data_idx, str(name_list))

            self.write(data_id, name_list, expect_diff)

    def write(self, data_id, name_list, expect_diff):
        hit = True
        with self.data_lock:
            if data_id not in self.id_table.keys():
                hit = False
                self.miss += 1
            else:
                logging.info("data %d with %s hit", data_id, name_list)
                self.replacer.delete(data_id)
                self.hit += 1
            
        if hit is False:
            logging.info("data %d with %s miss", data_id, name_list)
            if self._merge_pendingid(data_id, name_list, expect_diff):
                # p_ticker = m.tiker("pool")
                # p_ticker.start()
                data_idx = self.allocate_datanode()
                while data_idx == -1:
                    time.sleep(0.0001)
                    data_idx = self.allocate_datanode()
                self.id_queue.put((data_id, data_idx))
            return
        
        with self.data_lock:
            for name in name_list:
                data_idx = self.id_table[data_id]
                inode_idx = self.allocate_inode()
                if data_id not in self.data_refs.keys():
                    self.data_refs[data_id] = []
                self.data_refs[data_id].append(inode_idx)
                self.buffer.write_inode(inode_idx, self.task_tails[name], data_idx)
                logging.info("wirte %s's data [%d]-->[%d]-->(%d)", name, self.task_tails[name], inode_idx, data_idx)
                self.task_tails[name] = inode_idx
        
        self.replacer.update(data_id, expect_diff)
        # w_ticker = m.tiker("data write")
        # w_ticker.end()
        # w_ticker.print_avg(128, 128)
    
    def _merge_pendingid(self, data_id, name_list, expect_diff):
        res = False
        with self.pending_id_lock:
            if data_id not in self.pending_id.keys():
                res = True
                self.pending_id[data_id] = [[], 0]
            self.pending_id[data_id][0].extend(name_list)
            self.pending_id[data_id][1] = expect_diff
        return res
    
    def add_task(self, task_name):
        if task_name in self.task_heads.keys():
            return -1
        inode_idx = self.allocate_inode()
        self.buffer.write_inode(inode_idx)
        self.task_heads[task_name] = inode_idx
        self.task_tails[task_name] = inode_idx

        logging.info("add task %s with head %d", task_name, inode_idx)
        return inode_idx

    def allocate_inode(self):
        inode_idx = self.buffer.allocate_inode()
        if inode_idx != -1:
            return inode_idx

        # free some inode
        while True:
            for task_name in self.task_heads.keys():
                head_inode = self.task_heads[task_name]
                # print("try to free %d (%s)"%(head_inode, task_name))
                if self.buffer.is_used(head_inode) == False:
                    _, next_head = self.buffer.parse_inode(head_inode)
                    self.task_heads[task_name] = next_head
                    return head_inode

    def allocate_datanode(self):
        datanode_idx = self.buffer.allocate_datanode()
        if datanode_idx != -1:
            return datanode_idx

        # free some datanode
        valid = True
        with self.data_lock:
            data_id = self.replacer.next()
            while data_id != -1:
                data_idx = self.id_table[data_id]
                for ref in self.data_refs[data_id]:
                    valid = self.buffer.is_datavalid(ref, data_idx)
                    if valid is True:
                        break
                        
                if valid is False:
                    logging.info("evict data %d in %d", data_id, data_idx)
                    del self.id_table[data_id]
                    del self.data_refs[data_id]
                    self.replacer.delete(data_id)
                    self.replacer.reset()
                    return data_idx
        
        self.replacer.reset()
        return -1
    def delete_task(self, name):
        with self.pending_id_lock:
            for data_id in self.pending_id.keys():
                self.pending_id[data_id].remove(name)

        head = self.task_heads[name]

        with self.data_lock:
            del self.task_heads[name]
        
        while head != -1:
            # print("del", head)
            head = self.buffer.get_next(head)
        
    def terminate(self):
        self.loader.kill()
        while self.loader.is_alive() == True:
            time.sleep(0.1)
        self.loader.close()

import time
n = 1000000


def writer(bm, name_list,):
    from SamplingTree import SamplingTree
    sa = SamplingTree()
    for i in range(len(name_list)):
        sa.insert(list(range(i*100, n+i*100)), name_list[i])
    while True:
        # s_ticker = m.tiker("idx sampling")
        # s_ticker.start()
        idx_dict, expect_diff = sa.sampling()
        # s_ticker.end()
        # s_ticker.print_avg(128, 128)
        if len(idx_dict.keys()) == 0:
            return
        for i in idx_dict.keys():
            # print("write", i, idx_dict[i], expect_diff[i])
            bm.write(i, idx_dict[i], expect_diff[i])
        
def reader(node, name):
    c = Buffer("xiejian", 602116)
    batch_size = 32
    for i in range(int(n/batch_size)):
        sum_t1 = 0
        sum_t2 = 0
        for j in range(batch_size):
            now = time.time()
            next_node = c.get_next(node)
            while next_node == -1:
                next_node = c.get_next(node)
            t1 = time.time() - now
            c.read(next_node)
            t2 = time.time() - now - t1
            sum_t1 += t1
            sum_t2 += t2
            node = next_node
            # print("read", node)
        if name == '0':
            print(sum_t1, sum_t2)
    # print(name, (time.time()-now)/n)
    

def multi_test(n):
    bm = BufferManger("xiejian", data_len=602116, create=True, size=602220*100+1000)
    pool = []
    names = []
    for i in range(n):
        head = bm.add_task(str(i))
        names.append(str(i))
        p = multiprocessing.Process(target=reader, args=(head, str(i)))
        pool.append(p)
        p.start()

    writer(bm, names)
    for p in pool:
        p.join()
    bm.terminate()


if __name__ == '__main__':
    multi_test(1)
    print("end.....")