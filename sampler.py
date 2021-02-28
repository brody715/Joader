import random
import queue
import time
import threading
import copy
from encode import *
from mylog import *
import signal, os, sys
import multiprocessing
from loader import Loader
from buffer import Buffer
class SubSampler(object):
    def __init__(self, name, idx_list=[], independent=False, cap=64):
        #TODO: 策略待选择
        self.independent = independent

        self.idx_lock = threading.Lock()
        self.undecided = idx_list
        self.decided = []

        self.data_size = len(idx_list)

        self.name = name

    def state(self):
        with self.idx_lock:
            if(len(self.undecided) == 0):
                return 0
            else:
                return 1

    def set_idxlist(self, idx_list):
        with self.idx_lock:
            self.undecided = idx_list

    def reset(self):
        with self.idx_lock:
            self.undecided.extend(self.decided)
            self.decided = []
    
    def _random_sampling(self):
        random.seed(time.time())
        i = random.randint(0, len(self.undecided)-1)
        idx = self.undecided[i]
        return idx

    def next_idx(self, idx):
        if (len(self.undecided) == 0):
            return -1
        
        if not self.independent:
            with self.idx_lock:
                if idx in self.undecided:
                    self.undecided.remove(idx)
                    self.decided.append(idx)
                    return idx
        
        idx = self._random_sampling()
        with self.idx_lock:
            self.undecided.remove(idx)
            self.decided.append(idx)
        return idx
    
    def __len__(self):
        return len(self.undecided)

class Sampler(object):
    def __init__(self, idx_queue=None, data_queue=None, cap = 2, name="xiejian", create=True, size = 1024*1024*16):
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

        # idx -> subs
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
            l = len(self.alive_subsampler)
            for i in range(l+1):
                if i == l or len(subs) < len(self.alive_subsampler[i]):
                    self.alive_subsampler.insert(i, subs)
                    break

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
        del self.task_tiker[subs]

    def _next_idx(self):
        idx = -1
        idx_dict = {}
        for i in range(len(self.alive_subsampler)):
            idx = self.alive_subsampler[i].next_idx(idx)
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
            
    def sampling_idx(self):
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
        loader = multiprocessing.Process(target=Loader.loading, args=(sa.idx_queue, sa.data_queue))
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
                cmd= task[1]

                logging.info("sampler receive a task : %s. %d", name, cmd)
                # 如果出现重复的name，或者name找不到，返回succ = False
                sa.clean()
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