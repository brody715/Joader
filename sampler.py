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
from buffer import Buffer


class SubSampler(object):
    def __init__(self, name, idx_list, independent=False):
        '''
            Args:
                name(str): the name of this subsampler
                idx_list(list): data index list
                subs(SubSampler): The subs that calulating intersection with idx_list
        '''
        self.name = name
        self.independent = independent

        self.length = len(idx_list)

        self.private_idx = idx_list
        self.public_idx = []

        self.public_dict = {}
        self.private_dict = {}

        self.private_cursor = 0
        self.public_cursor = 0
    
    def state(self, ):
        return self.length
    
    def intersection(self, subs):
        if subs != None:
            public_idx = []
            private_idx = []
            for idx in self.private_idx:
                if subs.has(idx):
                    public_idx.append(idx)
                else:
                    private_idx.append(idx)
            self.private_idx = private_idx
            self.public_idx = public_idx

        random.shuffle(self.private_idx)
        random.shuffle(self.public_idx)

        for i in range(len(self.private_idx)):
            self.private_dict[self.private_idx[i]] = i
        for i in range(len(self.public_idx)):
            self.public_dict[self.public_idx[i]] = i

    def has(self, idx):
        return idx in self.private_dict.keys() or idx in self.public_dict.keys()

    def reset(self, subs):
        self.private_idx.extend(self.public_idx)
        self.length = len(self.private_idx)

        self.private_cursor = 0
        self.public_cursor = 0
        self.private_dict.clear()
        self.public_dict.clear()

    def _random_sampling(self, idx_list):
        idx = random.choice(idx_list)
        idx_list.remove(idx)
        return idx

    def _independent_sampling(self):
        if random.uniform(0, 1) < (len(self.private_idx)-self.public_cursor)/(self.length):
            idx = self.private_idx[self.private_cursor]
            self.private_cursor += 1
        else:
            idx = self.public_idx[self.public_cursor]
            self.public_cursor += 1
        self.length -= 1
        return idx

    def _dependent_sampling(self, idx, l):
        if idx in self.public_dict.keys():
            i_idx = self.public_dict[idx]
            if self.length >= l and random.uniform(0, 1) < (1-self.length/l):
                idx = self.private_idx[self.private_cursor]
                # pub_c ... i_idx ...; pri_c
                # => pub_c ... pri_c ...; i_idx
                self.public_idx[i_idx], self.private_idx[self.private_cursor] = self.private_idx[self.private_cursor], self.public_idx[i_idx]
            self.public_idx[i_idx], self.public_idx[self.public_cursor] = self.public_idx[self.public_cursor], self.public_idx[i_idx]
            self.public_cursor += 1
        else:
            public_l = len(self.public_idx)-self.public_cursor
            if self.length <= l and random.uniform(0, 1) < (1-(l*(self.length-public_l))/(self.length*(l-public_l))):
                idx = self.public_idx[self.public_cursor]
                self.public_cursor += 1
            else:
                idx = self.private_idx[self.private_cursor]
                self.private_cursor += 1
        self.length -= 1
        return idx

    def next_idx(self, idx, l):
        if self.length == 0:
            return -1

        if self.independent or idx == -1:
            return self._independent_sampling()
        return self._dependent_sampling(idx, l)

    def __len__(self):
        return self.length


class Sampler(object):
    def __init__(self, idx_queue=None, data_queue=None, cap=2, name="xiejian", create=True, size=1024*1024*32):
        if idx_queue is None:
            idx_queue = multiprocessing.Manager().Queue(maxsize=cap)
        if data_queue is None:
            data_queue = multiprocessing.Manager().Queue(maxsize=cap)

        # 每个任务的存活时间为10s
        self.ttl = 10
        self.task_tiker = {}

        self.buffer = Buffer(name, create, size)
        self.buffer_name = self.buffer.name

        # 进程通信管道
        self.data_queue = data_queue
        self.idx_queue = idx_queue

        self.alive_subsampler = []
        self.zombie_subsampler = []
        self.subsampler_list_lock = threading.Lock()

        # {idx: subs_name}
        self.pending_idx = {}
        self.pending_idx_lock = threading.Lock()

        # 当所有进程都已经sampling完毕，就block住，防止空转
        self.blocking_sampling = threading.Condition()

    def add_subsampler(self, subs):
        head = self.buffer.add_task(subs.name)
        if head == -1:
            return head

        self.task_tiker[subs.name] = time.time()
        with self.subsampler_list_lock:
            if len(self.alive_subsampler) == 0:
                subs.intersection(None)
            else:
                subs.intersection(self.alive_subsampler[-1])
            self.alive_subsampler.append(subs)

        with self.blocking_sampling:
            self.blocking_sampling.notify()
        logging.info("add subsampler name %s", subs.name)
        return head

    def restore_subsampler(self, name):
        logging.info("sampler restore subs %s", name)

        subs = None
        with self.subsampler_list_lock:
            for i in range(len(self.zombie_subsampler)):
                if self.zombie_subsampler[i].name == name:
                    self.zombie_subsampler[i].reset()
                    subs = self.zombie_subsampler[i]
                    del self.zombie_subsampler[i]
                    break

        if subs is not None:
            self.buffer.delete_task(subs.name)
            return self.add_subsampler(subs)
        return -1

    def _name2subs(self, subsampler_list, name):
        for i in range(len(self.zombie_subsampler)):
            if self.zombie_subsampler[i].name == name:
                return self.zombie_subsampler[i]

    def delete_subsampler(self, name):
        logging.info("sampler delete subs %s", name)
        if name not in self.task_tiker.keys():
            return

        self.buffer.delete_task(name)
        with self.subsampler_list_lock:
            for i in range(len(self.alive_subsampler)):
                if self.alive_subsampler[i].name == name:
                    del self.alive_subsampler[i]
                    return
            for i in range(len(self.zombie_subsampler)):
                if self.zombie_subsampler[i].name == name:
                    del self.zombie_subsampler[i]
                    return
        del self.task_tiker[name]

    def _next_idx(self):
        idx = -1
        idx_dict = {}
        l = 0
        for i in range(len(self.alive_subsampler)):
            idx = self.alive_subsampler[i].next_idx(idx, l)
            # l 应该+1, 在next_idx 中 已经-1，应该补全
            l = len(self.alive_subsampler[i])+1
            # idx = -1: idx全部消耗完毕
            if idx >= 0:
                if idx in idx_dict.keys():
                    idx_dict[idx].append(self.alive_subsampler[i].name)
                else:
                    idx_dict[idx] = [self.alive_subsampler[i].name]
        return idx_dict

    def _merge_idx(self, idx_dict):
        with self.pending_idx_lock:
            for i in idx_dict.keys():
                if i in self.pending_idx.keys():
                    self.pending_idx[i].extend(idx_dict[i])
                else:
                    self.pending_idx[i] = idx_dict[i]

    def clean(self):
        for subs_name in self.task_tiker.keys():
            if time.time() - self.task_tiker[subs_name] > self.ttl:
                logging.info("sampler expired %s", subs_name)
                self.delete_subsampler(subs_name)
        with self.subsampler_list_lock:
            for i in range(len(self.alive_subsampler)):
                if self.alive_subsampler[i].state() == 0:
                    self.zombie_subsampler.append(self.alive_subsampler[i])
                    del self.alive_subsampler[i]

    def update_tiker(self, name):
        logging.info("sampler update %s", name)
        if name not in self.task_tiker.keys():
            return -1
        self.task_tiker[name] = time.time()

    def dispatch_data(self, ):
        while True:
            try:
                item = self.data_queue.get()
            except:
                return
            idx, data = item
            # self.cache_cap.release()
            with self.pending_idx_lock:
                name_list = self.pending_idx[idx]
                self.buffer.write(data, name_list)

    def sampling_idx(self, ):
        while True:
            # self.cache_cap.acquire()
            with self.subsampler_list_lock:
                idx_dict = self._next_idx()

            if len(idx_dict.keys()) == 0:
                with self.blocking_sampling:
                    logging.info("sampling idx blocking")
                    self.blocking_sampling.wait()
                logging.info("sampling idx resuming")

            self._merge_idx(idx_dict)
            for i in idx_dict.keys():
                logging.critical("sampler put idx %d", i)
                try:
                    self.idx_queue.put(i)
                except:
                    return

    @staticmethod
    def sampler(task_queue, response_queue):
        logging.info("start sampler")

        sa = Sampler()

        # start loader process to load data
        loader = multiprocessing.Process(
            target=Loader.loading, args=(sa.idx_queue, sa.data_queue))
        loader.start()
        assert(loader.is_alive() == True)

        # start a thread to put index
        idx_sampler = threading.Thread(target=sa.sampling_idx, args=())
        idx_sampler.start()
        data_fetcher = threading.Thread(target=sa.dispatch_data, args=())
        data_fetcher.start()

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
                    subs = SubSampler(name, data)
                    succ = sa.add_subsampler(subs)
                elif(cmd == 1):
                    succ = sa.restore_subsampler(name)
                elif(cmd == -1):
                    succ = 1
                    sa.delete_subsampler(name)
                else:
                    sa.update_tiker(name)
                response_queue.put((name, (succ, sa.buffer_name)))
            except:
                loader.terminate()
                while loader.is_alive == True:
                    time.sleep(0.1)
                # assert(p.is_alive() != True)
                loader.close()
                print("sampler is exiting ......")
                return
