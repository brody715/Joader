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
class SubSampler(object):
    def __init__(self, name, idx_list=[], independent=False, cap=64):
        #TODO: 策略待选择
        self.cache = queue.Queue(maxsize=cap)

        self.independent = independent

        self.idx_lock = threading.Lock()
        self.undecided = idx_list
        self.decided = []
        self.pending_idx = []
        
        self.writer = threading.Thread(target=self.write, args=())
        self.writer.start()

        self.data_size = len(idx_list)

        self.name = name
        self.path = "/tmp/"+self.name
        
        logging.info("create subs: { %s: %s} ", name, self.path)
        
        assert(not os.path.exists(self.path))
        
        os.mkfifo(self.path)

        assert(os.path.exists(self.path))
        self.wf = os.open(self.path, os.O_CREAT | os.O_RDWR)
        
        self.alive_tiker = time.time()

    def update_tiker(self):
        self.alive_tiker = time.time()
    
    def expired(self, ttl):
        return (time.time()-self.alive_tiker) > ttl
    
    def set_idxlist(self, idx_list):
        with self.idx_lock:
            self.undecided = idx_list

    def reset(self):
        self.writer = threading.Thread(target=self.write, args=())
        self.writer.start()
        with self.idx_lock:
            self.undecided = []
            self.undecided.extend(self.decided)
            self.undecided.extend(self.pending_idx)
            self.decided = []
            self.pending_idx = []
            self.update_tiker()
    
    def _random_sampling(self):
        random.seed(time.time())
        i = random.randint(0, len(self.undecided)-1)
        idx = self.undecided[i]
        return idx

    def next_idx(self, idx_list):
        if (len(self.undecided) == 0):
            return False, -1
        
        if not self.independent:
            for idx in idx_list:
                if idx in self.undecided:
                    with self.idx_lock:
                        self.undecided.remove(idx)
                        self.pending_idx.append(idx)
                        self.update_tiker()
                    return False, idx
        
        idx = self._random_sampling()
        with self.idx_lock:
            self.undecided.remove(idx)
            self.pending_idx.append(idx)
            self.update_tiker()
        return True, idx
    
    def write(self):
        try:
            cnt = 0
            while True:
                data = self.cache.get(block=True)
                logging.critical("writing data")
                if data == -1:
                    break
                size_byte, data_byte = encode(data)
                os.write(self.wf, size_byte)
                os.write(self.wf, data_byte)
        except:
            return
        
    def send_data(self, idx, data, ttl=5):
        res = 0

        if idx in self.pending_idx:
            assert (data is not None)
            
            try:
                logging.critical("sampler put data %d length %d", idx, len(data))
                self.cache.put(data, block=True, timeout=ttl)
            except:
                return -1

            with self.idx_lock:
                self.decided.append(idx)
                self.pending_idx.remove(idx)
                self.update_tiker()
                if len(self.pending_idx)+len(self.undecided) == 0:
                    res = -1
        
        return res

    def __len__(self):
        return len(self.undecided)
    
    def zombie(self):
        self.cache.put(-1)

    def delete(self):
        self.zombie()
        os.close(self.wf)
        os.remove(self.path)

class Sampler(object):
    def __init__(self, idx_queue=None, data_queue=None, cap = 64):
        if idx_queue is None:
            idx_queue = multiprocessing.Manager().Queue(maxsize=cap)
        if data_queue is None:
            data_queue = multiprocessing.Manager().Queue(maxsize=cap)
        
        # 进程通信管道
        self.data_queue = data_queue
        self.idx_queue = idx_queue
        
        self.alive_subsampler = []
        self.zombie_subsampler = []
        self.subsampler_list_lock = threading.Lock()

        # idx -> subs
        self.pending_idx = {}
        self.pending_idx_lock = threading.Lock()
        
        # 负载均衡
        # self.cache_cap = threading.Semaphore(cap)

        # 当所有进程都已经sampling完毕，就block住，防止空转
        self.blocking_sampling = threading.Condition()
    
    def check_exist(self, name):
        with self.subsampler_list_lock:
            logging.info("check_exist")
            for i in range(len(self.alive_subsampler)):
                if self.alive_subsampler[i].name == name:
                    return True
            for i in range(len(self.zombie_subsampler)):
                if self.zombie_subsampler[i].name == name:
                    return True
        return False
    
    def add_subsampler(self, subs):
        with self.subsampler_list_lock:
            _subsampler_list = []
            for i in range(len(self.alive_subsampler)):
                if len(subs) < len(self.alive_subsampler[i]):
                    _subsampler_list.append(subs)
                    _subsampler_list.extend(self.alive_subsampler[i:])
                    return _subsampler_list, _dict
                _subsampler_list.append(self.alive_subsampler[i])
            _subsampler_list.append(subs)
        
            self.alive_subsampler = _subsampler_list
        
        with self.blocking_sampling:
            self.blocking_sampling.notify()
        logging.info("add subsampler name %s", subs.name)
        return True

    def restore_subsampler(self, name):
        logging.info("sampler restore subs %s", name)
        subs = None
        with self.subsampler_list_lock:
            restore_subs = None
            for i in range(len(self.zombie_subsampler)):
                if self.zombie_subsampler[i].name == name:
                    self.zombie_subsampler[i].reset()
                    subs = self.zombie_subsampler[i]
                    del self.zombie_subsampler[i]
                    break
        
        if subs == None:
            return False
        return self.add_subsampler(subs)

    
    def _name2subs(self, subsampler_list, name):
        for i in range(len(self.zombie_subsampler)):
            if self.zombie_subsampler[i].name == name:
                return self.zombie_subsampler[i]

    def delete_subsampler(self, name):
        logging.info("sampler delete subs %s", name)
        with self.subsampler_list_lock:
            for i in range(len(self.alive_subsampler)):
                if self.alive_subsampler[i].name == name:
                    self.alive_subsampler[i].delete()
                    del self.alive2zombie[i]
                    return True
            for i in range(len(self.zombie_subsampler)):
                if self.zombie_subsampler[i].name == name:
                    self.zombie_subsampler[i].delete()
                    del self.zombie_subsampler[i]
                    return True
        return False

    def _next_idx(self):
        l = len(self.alive_subsampler)
        idx_dict = {}

        idx_list = []
        for i in range(l):
            independent, idx = self.alive_subsampler[i].next_idx(idx_list)
            if independent:
                idx_list.append(idx)
            
            # idx = -1: idx全部消耗完毕
            if idx >= 0:
                if idx in idx_dict.keys():
                    idx_dict[idx].append(self.alive_subsampler[i])
                else:
                    idx_dict[idx] = [self.alive_subsampler[i]]
        return idx_dict

    def _merge_idx(self, idx_dict):
        with self.pending_idx_lock:
            for i in idx_dict.keys():
                if i in self.pending_idx.keys():
                    self.pending_idx[i].extend(idx_dict[i])
                else:
                    self.pending_idx[i] = idx_dict[i]

    def alive2zombie(self, subs):
        subs.zombie()
        with self.subsampler_list_lock:
            for i in range(len(self.alive_subsampler)):
                if subs.name == self.alive_subsampler[i].name:
                    self.zombie_subsampler.append(self.alive_subsampler[i])
                    del self.alive_subsampler[i]
                    break

    def dispatch_data(self, ):
        while True:
            item = self.data_queue.get(True)
            idx, data = item
            # self.cache_cap.release()
            
            with self.pending_idx_lock:
                for subs in self.pending_idx[idx]:
                    err = subs.send_data(idx, data)
                    if err == -1:
                        self.alive2zombie(subs)


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
                self.idx_queue.put(i)
    
    def check_expired(self, ttl = 5):
        with self.subsampler_list_lock:
            i = 0
            while i < len(self.alive_subsampler):
                if self.alive_subsampler[i].expired(ttl):
                    logging.info("sampler %s expired", self.alive_subsampler[i].name)
                    self.alive_subsampler[i].delete()
                    del self.alive_subsampler[i]
                else:
                    i += 1
            i = 0
            while i < len(self.zombie_subsampler):
                if self.zombie_subsampler[i].expired(ttl):
                    logging.info("sampler %s expired", self.zombie_subsampler[i].name)
                    self.zombie_subsampler[i].delete()
                    del self.zombie_subsampler[i]
                else:
                    i += 1
    
    def delete(self):
        with self.subsampler_list_lock:
            for i in range(len(self.alive_subsampler)):
                self.alive_subsampler[i].delete()
                del self.alive_subsampler[i]

            for i in range(len(self.zombie_subsampler)):
                self.zombie_subsampler[i].delete()
                del self.zombie_subsampler[i]

    @staticmethod
    def sampler(task_queue, response_queuee):
        logging.info("start sampler")

        sa = Sampler()
        
        # start loader process
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
                name = list(task.keys())[0]
                data = list(task.values())[0]
                logging.info("sampler receive a task : %s", name)
                
                sa.check_expired()
                
                # 如果出现重复的name，或者name找不到，返回succ = False
                if (type(data) == list):
                    if sa.check_exist(name):
                        succ = False
                    else:
                        subs = SubSampler(name, data)
                        succ = sa.add_subsampler(subs)
                elif (type(data) == int):
                    if data >= 0:
                        # TODO: error process
                        succ = sa.restore_subsampler(name)
                    else:
                        succ = sa.delete_subsampler(name)
                
                if succ:
                    response_queue.put({name:b'1'})
                else:
                    response_queue.put({name:b'0'})
            except:
                sa.delete()
                loader.terminate()
                while loader.is_alive == True:
                    time.sleep(0.1)
                # assert(p.is_alive() != True)
                loader.close()
                print("sampler is exiting ......")
                return

# sa = SubSampler("xx", [1,2,3,4])
# sa.delete()
# def test():
#     s = SubSampler([1,2,3,4,5,6])
#     print(s.next_idx())

# test()