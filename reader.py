import reader
import dataset
import time
import multiprocessing
import queue
from multiprocessing import Process, Queue, Pool, Semaphore
import threading
from mylog import *


#TODO：这个需要手动定义，有点丑陋
ds = dataset.lmdbDataset('/data/share/ImageNet/ILSVRC-train.lmdb', True)

class Reader(object):
    @staticmethod
    def process(idx_queue, data_queue):
        for idx in range(10):
            data = ds[idx]
            # data_queue.put((idx, data))
    
        # while True:
        #     now = time.time()
        #     logging.critical("reader try idx")
        #     idx = idx_queue.get(True)
            
        #     data = ds[idx]
        #     logging.critical("reader put data %d", idx)
        #     # data_queue.put((idx, data))
    
    @staticmethod
    def read_data(idx_queue, middle_queue, ds, sem):
        while True:
            idx = idx_queue.get(True)
            sem.acquire()
            logging.info("reader acuqire sem %s", sem)
            data = ds[idx]
            logging.critical("reader read idx %d", idx)
            middle_queue.put((idx, data))

    @staticmethod
    def stop_pool(mp):
        mp.close()
        mp.join()
    @staticmethod
    #TODO: hard code
    def reader(idx_queue, data_queue, workers=8, s=0):
        logging.info("start reader")
        
        # middle_queue = queue.Queue()
        if workers == 0:
            workers = multiprocessing.cpu_count()
        if s == 0:
            s = 2*workers
        p = Pool(processes = workers)
        sem = Semaphore(s)
        
        # thred = threading.Thread(target=Reader.read_data, args=(idx_queue, middle_queue, ds, sem))
        # thred.start()

        with 
        p.close()
        p.join()