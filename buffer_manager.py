from multiprocessing import Manager
from sampling_tree import SamplingTree
import time
import signal
import os
import sys
import multiprocessing
import queue
import threading
import mmap
import config as cfg
from mylog import logging
from loader import Loader
from replacer import Replacer
from buffer import Buffer
from monitor import *


class BufferManger(object):
    def __init__(self, in_queue, out_queue):
        # create buffer
        self.buffer = Buffer(cfg.MMAP_FILE_PATH, cfg.DATASIZE,
                             create=True, size=cfg.BUFFERSIZE)
        self.buffer_path = cfg.MMAP_FILE_PATH

        self.in_queue = in_queue
        self.out_queue = out_queue

        # table
        # protect id_table data_refs task tails task heads
        self.data_lock = threading.Lock()
        self.id_table = {}  # id -> datanode
        self.data_refs = {}  # id -> refs
        self.task_tails = {}
        self.task_heads = {}

        self.pending_id_lock = threading.Lock()
        self.pending_id = {}  # id->namelist

        # replacer
        self.replacer = Replacer()

        # monitor
        self.hit = 0
        self.miss = 0

    def listen(self):
        while True:
            item = self.in_queue.get()
            data_id, data_addr = item
            with self.pending_id_lock:
                name_list, expect_diff = self.pending_id[data_id]
                del self.pending_id[data_id]
            with self.data_lock:
                self.id_table[data_id] = data_addr
            logging.info("buffer manager get data %d in %d with tasks %s",
                         data_id, data_addr, str(name_list))
            self.write(data_id, name_list, expect_diff)

    def _write_inodes(self, data_id, name_list):
        with self.data_lock:
            for name in name_list:
                data_addr = self.id_table[data_id]
                inode_addr = self.allocate_inode()
                # TODO: Infinite loop
                while inode_addr == -1:
                    inode_addr = self.allocate_inode()
                self.buffer.write_inode(
                    inode_addr, self.task_tails[name], data_addr)
                logging.info("wirte %s's data [%d]-->[%d]-->(%d)",
                             name, self.task_tails[name], inode_addr, data_addr)
                if data_id not in self.data_refs.keys():
                    self.data_refs[data_id] = []
                self.data_refs[data_id].append(inode_addr)
                self.task_tails[name] = inode_addr
    
    def write(self, data_id, name_list, expect_diff):
        hit = False
        with self.data_lock:
            if data_id in self.id_table.keys():
                self.replacer.pin(data_id)
                hit = True
        if hit:
            logging.info("data %d with %s hit", data_id, name_list)
            self.hit += 1
            self._write_inodes(data_id, name_list)
            logging.info("update %d: %d", data_id, expect_diff)
            self.replacer.update(data_id, expect_diff)
        else:
            logging.info("data %d with %s miss", data_id, name_list)
            if self._merge_pendingid(data_id, name_list, expect_diff): 
                with self.data_lock:
                    data_addr = self.allocate_datanode()
                # TODO: Infinite loop
                while data_addr == -1:
                    with self.data_lock:
                        data_addr = self.allocate_datanode()
                self.out_queue.put((data_id, data_addr))

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
        with self.data_lock:
            self.task_heads[task_name] = inode_idx
            self.task_tails[task_name] = inode_idx

        logging.info("add task %s with head %d", task_name, inode_idx)
        return inode_idx

    def allocate_inode(self):
        inode_idx = self.buffer.allocate_inode()
        if inode_idx != -1:
            return inode_idx

        # free some inode
        for task_name in self.task_heads.keys():
            head_inode = self.task_heads[task_name]
            if self.buffer.is_used(head_inode) == False:
                _, next_head = self.buffer.parse_inode(head_inode)
                self.task_heads[task_name] = next_head
                return head_inode
        return -1

    def allocate_datanode(self):
        datanode_idx = self.buffer.allocate_datanode()
        if datanode_idx != -1:
            return datanode_idx

        # free some datanode
        data_id = self.replacer.next()
        if data_id == -1:
            return data_id
        
        data_addr = self.id_table[data_id]
        for ref in self.data_refs[data_id]:
            valid = self.buffer.is_datavalid(ref, data_addr)
            if valid is True:
                return -1

        logging.info("evict data %d in %d", data_id, data_addr)
        del self.id_table[data_id]
        del self.data_refs[data_id]
        self.replacer.pin(data_id)
        return data_addr

    def delete_task(self, name):
        with self.pending_id_lock:
            for data_id in self.pending_id.keys():
                self.pending_id[data_id].remove(name)

        with self.data_lock:
            head = self.task_heads[name]
            del self.task_heads[name]

        while head != -1:
            head = self.buffer.get_next(head)
        logging.info("buffer manager deleta task %s", name)


n = 100000


def writer(bm, name_list,):
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
    c = Buffer(cfg.MMAP_FILE_PATH, cfg.DATASIZE)
    batch_size = 32
    for _ in range(int(n/batch_size)):
        sum_t1 = 0
        sum_t2 = 0
        for _ in range(batch_size):
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
    bm_in_queue = Manager().Queue(cfg.QUEUE_SIZE)
    bm_out_queue = Manager().Queue(cfg.QUEUE_SIZE)
    bm = BufferManger(bm_in_queue, bm_out_queue)
    l = Loader(bm_out_queue, bm_in_queue)
    l.start()
    pool = []
    names = []
    for i in range(n):
        head = bm.add_task(str(i))
        names.append(str(i))
        p = multiprocessing.Process(target=reader, args=(head, str(i)))
        pool.append(p)
        p.start()

    threading.Thread(target=bm.listen).start()
    for p in pool:
        p.join()
    l.terminate()


# if __name__ == '__main__':
#     multi_test(1)
#     print("end.....")
