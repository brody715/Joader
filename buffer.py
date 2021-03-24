import signal, os, sys
sys.path.append("/home/xj/proj/DM/pytorch-imagenet/Loader/")

import multiprocessing
import queue
import threading
from mylog import *
import mmap
from loader import Loader
from replacer import Replacer
from concurrent.futures import ThreadPoolExecutor
from monitor import *

class BufferManger(object):
    def __init__(self, name, cap=16, create=False, size=0):
    
        # start loader service
        self.id_queue = multiprocessing.Manager().Queue(maxsize=cap)
        self.data_queue = multiprocessing.Manager().Queue(maxsize=cap)
        self.loader = multiprocessing.Process(
            target=Loader.loading, args=(self.id_queue, self.data_queue, cap))
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
        #buffer
        self.buffer = Buffer(name, create, size)

        # start a thread to listen data queue
        executor = ThreadPoolExecutor(max_workers=cap)
        executor.submit(self.listener)

    def listener(self):
        while True:
            # try:
            item = self.data_queue.get()
            p_ticker = m.tiker("data process")
            p_ticker.end()
            p_ticker.print_avg(128, 128)
            # except:
            #     logging.error("listener read data queue error")
            #     exit(0)
            w_ticker = m.tiker("data write")
            w_ticker.start()

            data_id, data = item
            logging.info("buffer get data %d with length %d", data_id, len(data))
            name_list = []
            with self.pending_id_lock:
                if data_id in self.pending_id.keys():
                    name_list, expect_diff = self.pending_id[data_id]
                    del self.pending_id[data_id]
            if len(name_list) == 0:
                continue

            data_idx = self.write_data(data)
            logging.info("buffer write data %d in %d with tasks %s", data_id, data_idx, str(name_list))
            with self.data_lock:
                self.id_table[data_id] = data_idx
            
            self.write(data_id, name_list, expect_diff)

    def write_data(self, data):
        if self.buffer.DATA_LEN == -1:
            self.buffer.set_datalen(len(data))
        data_idx = self.allocate_datanode()
        self.buffer.write_data(data_idx, data)
        return data_idx

    def write(self, data_id, name_list, expect_diff):
        hit = True
        with self.data_lock:
            if data_id not in self.id_table.keys():
                hit = False
            else:
                logging.info("data %d with %s hit", data_id, name_list)
                self.replacer.delete(data_id)
        
        if hit is False:
            logging.info("data %d with %s miss", data_id, name_list)
            if self._merge_pendingid(data_id, name_list, expect_diff):
                p_ticker = m.tiker("pool")
                p_ticker.start()
                self.id_queue.put(data_id)
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
        
        w_ticker = m.tiker("data write")
        w_ticker.end()
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
        while True:
            # print("find datanode")
            with self.data_lock:
                data_id = self.replacer.next()
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

class Buffer(object):
    def __init__(self, name, create=False, size=0):
        if create:
            assert(size != 0)
            path = "/tmp/"+name
            f = open(path, "wb+")
            f.write(b'0'*size)
            self.buf = mmap.mmap(f.fileno(), size)
            self.buf.flush()
        else:
            path = "/tmp/"+name
            f = open(path, "rb+")
            self.buf = mmap.mmap(f.fileno(), size)

        self.create = create
        self.size = size
        self.name = name

        # | -- inode -->     <-- data --|
        self.inode_tail = 0
        self.data_head = size


        # basic config
        # | VALID BYTE | DATA_IDX | NEXT_IDX |
        self.INDEX_LEN = 4
        self.VALID_LEN = 3
        self.INODE_LEN = self.VALID_LEN + 2*self.INDEX_LEN

        self.VALID_OFF = 0
        self.DATA_OFF = self.VALID_OFF + 0
        self.NEXT_OFF = self.VALID_OFF + 1
        self.USED_OFF = self.VALID_OFF + 2

        self.DATA_IDX_OFF = self.VALID_LEN
        self.NEXT_IDX_OFF = self.VALID_LEN + self.INDEX_LEN

        # data = | datasize | data |
        self.DATASIZE_LEN = 4
        self.BYTE_ORDER = 'big'
        self.DATA_LEN = -1
    
    def set_datalen(self, length):
        self.DATA_LEN = length+self.DATASIZE_LEN
    
    def print_inode(self, idx):
        data_idx_byte = self.buf[idx + self.DATA_IDX_OFF:idx +
                                 self.DATA_IDX_OFF + self.INDEX_LEN]
        data_idx = int.from_bytes(data_idx_byte, self.BYTE_ORDER)
        next_idx_byte = self.buf[idx + self.NEXT_IDX_OFF:idx +
                                 self.NEXT_IDX_OFF + self.INDEX_LEN]
        next_idx = int.from_bytes(next_idx_byte, self.BYTE_ORDER)
        # print(idx, ":", self.buf[idx], data_idx, next_idx)

    def read(self, idx):
        data_idx_byte = self.buf[idx + self.DATA_IDX_OFF:idx +
                                 self.DATA_IDX_OFF + self.INDEX_LEN]
        data_idx = int.from_bytes(data_idx_byte, self.BYTE_ORDER)

        datasize_byte = self.buf[data_idx:data_idx + self.DATASIZE_LEN]
        datasize = int.from_bytes(datasize_byte, self.BYTE_ORDER)

        data_byte = self.buf[data_idx + self.DATASIZE_LEN:data_idx +
                             self.DATASIZE_LEN + datasize]

        self.buf[idx + self.DATA_OFF] = 0
        # print(idx, " delete", data_idx)
        # logging.info("read data(%d) inode %d in %s", data_idx, idx, self.name)
        return data_byte
    
    def parse_inode(self, idx):
        data_idx_byte = self.buf[idx + self.DATA_IDX_OFF:idx +
                                 self.DATA_IDX_OFF + self.INDEX_LEN]
        data_idx = int.from_bytes(data_idx_byte, self.BYTE_ORDER)
        next_idx_byte = self.buf[idx + self.NEXT_IDX_OFF:idx +
                                 self.NEXT_IDX_OFF + self.INDEX_LEN]
        next_idx = int.from_bytes(next_idx_byte, self.BYTE_ORDER)
        return data_idx, next_idx
    
    def get_next(self, idx):
        if (self.buf[idx + self.NEXT_OFF]) == 0 or (self.buf[idx + self.USED_OFF]) == 0:
            return -1
        next_idx_byte = self.buf[idx + self.NEXT_IDX_OFF:idx +
                                 self.NEXT_IDX_OFF + self.INDEX_LEN]
        next_idx = int.from_bytes(next_idx_byte, self.BYTE_ORDER)
        self.buf[idx + self.USED_OFF] = 0
        return next_idx

    def is_used(self, inode_idx):
        return self.buf[inode_idx+self.USED_OFF] == 1

    def is_datavalid(self, inode_idx, data_idx):
        _data_idx = int.from_bytes(
                        self.buf[inode_idx + self.DATA_IDX_OFF:inode_idx +
                                 self.DATA_IDX_OFF + self.INDEX_LEN],
                        self.BYTE_ORDER)
        return self.buf[inode_idx + self.DATA_OFF] != 0 and _data_idx == data_idx

    def allocate_inode(self):
        if self.inode_tail + self.INODE_LEN < self.data_head:
            idx = self.inode_tail
            self.inode_tail += self.INODE_LEN
            self.buf[idx+self.DATA_OFF] = 0
            self.buf[idx+self.USED_OFF] = 0
            self.buf[idx+self.NEXT_OFF] = 0
            return idx
        return -1

    def allocate_datanode(self):
        if self.data_head - self.DATA_LEN > self.inode_tail:
            self.data_head = self.data_head - self.DATA_LEN
            return self.data_head
        return -1

    def write_inode(self, curnode_idx, lastnode_idx=-1, data_idx=-1):
        self.buf[curnode_idx+self.USED_OFF] = 1
        self.buf[curnode_idx+self.DATA_OFF] = 0
        self.buf[curnode_idx+self.NEXT_OFF] = 0
    
        if data_idx != -1:
            data_idx_byte = data_idx.to_bytes(self.INDEX_LEN, self.BYTE_ORDER)

            # copy this data idx
            self.buf[curnode_idx + self.DATA_IDX_OFF:curnode_idx +
                    self.DATA_IDX_OFF + self.INDEX_LEN] = data_idx_byte
            self.buf[curnode_idx + self.DATA_OFF] = 1

        # link last idx
        if lastnode_idx != -1:
            curnode_idx_byte = curnode_idx.to_bytes(self.INDEX_LEN,
                                                        self.BYTE_ORDER)
            self.buf[lastnode_idx + self.NEXT_IDX_OFF:lastnode_idx +
                    self.NEXT_IDX_OFF + self.INDEX_LEN] = curnode_idx_byte
            self.buf[lastnode_idx + self.NEXT_OFF] = 1

        return curnode_idx

    def write_data(self, data_idx, data):
        size_byte = len(data).to_bytes(self.DATASIZE_LEN,
                                       byteorder=self.BYTE_ORDER)
        # write data
        assert(len(size_byte+data) == self.DATA_LEN)
        self.buf[data_idx:data_idx + self.DATA_LEN] = size_byte + data

        return data_idx

    def debug_print(self, ):
        for i in range(0, self.inode_tail, self.INODE_LEN):
            print(self.buf[i],
                  int.from_bytes(self.buf[i + 1:i + 5], self.BYTE_ORDER),
                  int.from_bytes(self.buf[i + 5:i + 9], self.BYTE_ORDER),
                  end=' | ')

    def __del__(self, ):
        pass
        # self.shm.close()
        # self.shm.unlink()


import time
n = 1000000


def writer(bm, name_list,):
    from SamplingTree import SamplingTree
    sa = SamplingTree()
    for i in range(len(name_list)):
        sa.insert(list(range(i*100, n+i*100)), name_list[i])
    while True:
        s_ticker = m.tiker("idx sampling")
        s_ticker.start()
        idx_dict, expect_diff = sa.sampling()
        s_ticker.end()
        # s_ticker.print_avg(128, 128)
        if len(idx_dict.keys()) == 0:
            return
        for i in idx_dict.keys():
            # print("write", i, idx_dict[i], expect_diff[i])
            bm.write(i, idx_dict[i], expect_diff[i])
        
def reader(node, name):
    
    c = Buffer("xiejian")
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
        # if name == '0':
        #     print(sum_t1, sum_t2)
        time.sleep(0.08)
    # print(name, (time.time()-now)/n)
    

def multi_test(n):
    bm = BufferManger("xiejian", create=True, size=602220*100+1000)
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
    multi_test(8)
    print("end.....")