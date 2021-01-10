import random
import time
import threading
import os
from cache import Cache
import copy
import logging
from encode import *

LOG_FORMAT = "[%(asctime)s]%(levelname)s : %(message)s"
logging.basicConfig(level=logging.DEBUG, format=LOG_FORMAT)

class SubSampler(object):
    def __init__(self, name, idx_list=[]):
        #TODO: 策略待选择
        self.undecided = idx_list
        self.decided = []
        self.pending_idx = []

        self.name = name
        self.path = "/tmp/"+self.name
        if os.path.exists(self.path):
            #TODO : 报错
            os.remove(self.path)
        logging.info("create subs: { %s: %s} ", name, self.path)
        os.mkfifo(self.path)

        self.wf = os.open(self.path, os.O_SYNC | os.O_CREAT | os.O_RDWR)
    def set_idxlist(self, idx_list):
        self.undecided = idx_list
    
    def reset(self):
        self.undecided = []
        self.undecided.extend(self.decided)
        self.undecided.extend(self.pending_idx)
        self.decided = []
        self.pending_idx = []
    
    def next_idx(self, idx):
        if (len(self.pending_idx) + len(self.undecided) == 0):
            return -2
        if (len(self.undecided) == 0):
            return -1
        if idx in self.undecided:
            return idx
        
        i = random.randint(0, len(self.undecided)-1)
        idx = self.undecided[i]
        self.undecided.pop(i)
        self.pending_idx.append(idx)
        
        return idx
    
    def put_data(self, cache, thresh=25*1024*1024,): # 25M
        for idx in self.pending_idx:
            if os.path.getsize(self.path) > thresh:
                break
            if cache.has(idx):
                data = cache.get(idx)
                assert (data is not None)
                data = encode(data)
                logging.info("sampler put data idx %d", idx)
                os.write(self.wf, data)
                self.decided.append(idx)
                self.pending_idx.remove(idx)
    
    def __len__():
        return len(self.undecided)
    
    def __del__(self):
        os.close(self.wf)
        os.remove(self.path)

class Sampler(object):
    def __init__(self, idx_queue, data_queue):
        self.subsampler_list = []
        self.idx_dict = {} # name: idx

        # 当插入新的subs到list中去的时候，先不立即插入而是放入缓存区内
        self.pending_subs = []
        self.pending_name = []
        self.pending_lock = threading.Lock()
        
        self.free_list = []
        
        self.cache = Cache()

        # 进程通信管道
        self.idx_queue = idx_queue
        self.data_queue = data_queue

    def try_add_subsampler(self, subs):
        with self.pending_lock:
            logging.info("sampler append to pending subs: %s", subs.name)
            self.pending_subs.append(subs)
    def try_add_subsamplername(self, name):
        with self.pending_lock:
            logging.info("sampler append to pending name: %s", name)
            self.pending_name.append(name)
    
    def get_idx_byname(self, subs_list, name):
        for i in range(len(subs_list)):
            if subs.name == name:
                return i
        return -1
    
    def _add_allsampler(self):
        pending_subs = []
        # TODO: 待优化，批量合并？
        with self.pending_lock:
            for subs in self.pending_subs:
                logging.info("add subs %s", subs.name)
                self.subsampler_list, self.idx_dict = self._add_subsampler(subs)
            for name in self.pending_name:
                i = self.get_idx_byname(self.free_list, name)
                subs = self.free_list[i]
                self.subsampler_list, self.idx_dict = self._add_subsampler(subs)
                self.free_list.pop(i)
            
            self.pending_name = []
            self.pending_subs = []


    def _add_subsampler(self, subs):
        # insert sort
        _dict = {}
        _subsampler_list = []

        for i in range(len(self.subsampler_list)):
            if len(subs) < len(self.subsampler_list[i]):
                _subsampler_list.append(subs)
                _dict[subs.name] = i
                for j in range(i, len(self.subsampler_list)):
                    _subsampler_list.append(subsampler_list[i])
                    _dict[subsampler_list[i].name] = j+1
                return _subsampler_list, _dict
            
            _subsampler_list.append(subsampler_list[i])
            _dict[subsampler_list[i].name] = i
        
        _subsampler_list.append(subs)
        _dict[subs.name] = len(_subsampler_list)-1
    
        return _subsampler_list, _dict
    
    def delete(self, i):
        subs = self.subsampler_list[i]
        subs.reset()
        self.free_list.append(subs)
        self.subsampler_list.pop(i)

    def release(self, name):
        i = self.get_idx_byname(self.free_list, name)
        if i != -1:
            self.free_list.pop(i)

    def next_idx(self):
        l = len(self.subsampler_list)
        idx = -1
        idx_dict = {}
        self._add_allsampler()
        for i in range(l):
            idx = self.subsampler_list[i].next_idx(idx)
            # idx = -2: 已经读完一个epoch
            # idx = -1: 读完idx，但是还没有读完数据
            # idx >= 0: 未读完
            if idx == -2:
                self.delete(i)
            elif idx >= 0:
                if idx in idx_dict.keys():
                    idx_dict[idx] += 1
                else:
                    idx_dict[idx] = 0
        return idx_dict

    def put_data(self):
        while True:
            l = len(self.subsampler_list)
            for i in range(l):
                self.subsampler_list[i].put_data(self.cache)
            time.sleep(0.5)

    def sampling_idx(self):
        while True:
            idx_dict = self.next_idx()
            self.cache.merge_index(idx_dict)

            for i in idx_dict.keys():
                self.idx_queue.put(i)
            
            time.sleep(0.5)
    
    def fetch_data(self, ):
        while True:
            item = self.data_queue.get(True)
            idx = item[0]
            data = item[1]
            logging.info("sampler fetch idx %d", idx)
            self.cache.set(idx, data)

    @staticmethod
    def sampler(task_queue, idx_queue, data_queue):
        sa = Sampler(idx_queue, data_queue)
        
        # start a thread to put index
        idx_sampler = threading.Thread(target=sa.sampling_idx, args=())
        idx_sampler.start()

        data_fetcher = threading.Thread(target=sa.fetch_data, args=())
        data_fetcher.start()

        data_putter = threading.Thread(target=sa.put_data, args=())
        data_putter.start()

        while True:
            
            task = task_queue.get(True)
            name = list(task.keys())[0]
            data = list(task.values())[0]
            logging.info("sampler receive a task from data queue: %s", name)
            if (type(data) == list):
                subs = SubSampler(name, data)
                sa.try_add_subsampler(subs)
            elif (type(data) == int):
                if data >= 0:
                    # TODO: error process
                    sa.try_add_subsamplername(name)
                else:
                    sa.release(name)

# def test():
#     s = SubSampler([1,2,3,4,5,6])
#     print(s.next_idx())

# test()