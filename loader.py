import loader
import dataset
import time
import multiprocessing
import queue
import threading
from mylog import *
import torch


#TODO：这个需要手动定义，有点丑陋
ds = dataset.lmdbDataset('/data/share/ImageNet/ILSVRC-train.lmdb', True)

class Loader(object):
    @staticmethod
    def process(idx_queue, data_queue):
        torch.set_num_threads(1)
        while True:
            idx = idx_queue.get(True)
            logging.critical("loader get idx %d,", idx)
            data = ds[idx]
            logging.critical("loader put data %d,", idx)
            data_queue.put((idx, data))

    @staticmethod
    def stop_pool(mp):
        mp.close()
        mp.join()
    @staticmethod
    #TODO: hard code workers
    def loading(idx_queue, data_queue, workers=8, s=0):
        logging.info("start loader")
        
        # middle_queue = queue.Queue()
        if workers == 0:
            workers = multiprocessing.cpu_count()
        if s == 0:
            s = 2*workers
        p = multiprocessing.Pool(processes = workers)
        sem = multiprocessing.Semaphore(s)
        
        for i in range(workers):
            p.apply_async(Loader.process, (idx_queue, data_queue))
        p.close()
        p.join()