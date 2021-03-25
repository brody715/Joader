import random
import queue
import time
import threading
import copy
from encode import *
from mylog import *
import signal
import os
import sys
import multiprocessing
from loader import Loader
from buffer_manager import BufferManger
from SamplingTree import SamplingTree


class Sampler(object):
    def __init__(self, name="xiejian", create=True, size=1024*1024*512):
        # 每个任务的存活时间为10s
        self.ttl = 10
        self.task_tiker = {}

        self.bm = BufferManger(name, create=True, size = size)
        self.buffer_name = name

        self.sampling_tree = SamplingTree()
        self.tree_lock = threading.Lock()

        # 当所有进程都已经sampling完毕，就block住，防止空转
        self.blocking_sampling = threading.Condition()

    def add_subsampler(self, name, idx_list):
        head = self.bm.add_task(name)
        if head == -1:
            return head

        self.task_tiker[name] = time.time()
        with self.tree_lock:
            self.sampling_tree.insert(idx_list, name)

        with self.blocking_sampling:
            self.blocking_sampling.notify()
        logging.info("add subsampler name %s", name)
        return head

    def delete_subsampler(self, name):
        logging.info("sampler delete subs %s", name)
        if name not in self.task_tiker.keys():
            return

        self.bm.delete_task(name)
        with self.tree_lock:
            self.sampling_tree.remove(name)
            
        del self.task_tiker[name]


    def clean(self):
        for subs_name in self.task_tiker.keys():
            if time.time() - self.task_tiker[subs_name] > self.ttl:
                logging.info("sampler expired %s", subs_name)
                self.delete_subsampler(subs_name)

    def update_tiker(self, name):
        logging.info("sampler update %s", name)
        if name not in self.task_tiker.keys():
            return -1
        self.task_tiker[name] = time.time()

    def sampling_idx(self, ):
        while True:
            with self.tree_lock:
                idx_dict, expect_diff = self.sampling_tree.sampling()
            if len(idx_dict) == 0:
                with self.blocking_sampling:
                    logging.info("sampling idx blocking")
                    self.blocking_sampling.wait()
                logging.info("sampling idx resuming")
            for i in idx_dict.keys():
                logging.critical("sampler put idx %d", i)
                self.bm.write(i, idx_dict[i], expect_diff[i])
                

    @staticmethod
    def sampler(task_queue, response_queue):
        logging.info("start sampler")

        sa = Sampler()

        # start a thread to put index
        idx_sampler = threading.Thread(target=sa.sampling_idx, args=())
        idx_sampler.start()

        while True:
            try:
                task = task_queue.get(True)
                name = task[0]
                cmd = task[1]

                logging.info("sampler receive a task : %s. %d", name, cmd)
                # 如果出现重复的name，或者name找不到，返回succ = False
                # sa.clean()
                if (cmd == 0):
                    data = task[2]
                    succ = sa.add_subsampler(name, data)
                elif(cmd == -1):
                    succ = 1
                    sa.delete_subsampler(name)
                else:
                    sa.update_tiker(name)
                response_queue.put((name, (succ, sa.buffer_name)))
            except:
                print("sampler is exiting ......")
                return

def test():
    sa = Sampler()
    sa.add_subsampler("aaa",list(range(100)))
    sa.add_subsampler("bbb",list(range(100)))
    idx_sampler = threading.Thread(target=sa.sampling_idx, args=())
    idx_sampler.start()
    idx_sampler.join()
if __name__ == '__main__':
    test()
    