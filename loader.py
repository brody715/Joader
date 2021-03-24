import loader
import dataset
import time
import multiprocessing
import queue
import threading
from mylog import *
import torch
import signal, os, sys


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
    #TODO: hard code workers
    def loading(idx_queue, data_queue, workers=8, s=0):
        logging.info("start loader")
        # middle_queue = queue.Queue()
        if workers == 0:
            workers = multiprocessing.cpu_count()
        pool = multiprocessing.Pool(processes = workers)
        try:
            for i in range(workers):
                pool.apply_async(Loader.process, (idx_queue, data_queue))
            pool.close()
            pool.join()
        except:
            print("loader is exiting ......")
            pool.close()
            pool.terminate()
            return

n = 1000
def put(idx_queue):
    for i in range(n):
        idx_queue.put(i)
def get(data_queue):
    t = time.time()
    for i in range(n):
        data_queue.get()
        print((time.time()-t)/(i+1))
def test():
    cap = 16
    id_queue = multiprocessing.Manager().Queue(maxsize=cap)
    data_queue = multiprocessing.Manager().Queue(maxsize=cap)
    loader = multiprocessing.Process(
        target=Loader.loading, args=(id_queue, data_queue))
    loader.start()

    threading.Thread(target=put, args=(id_queue,)).start()
    threading.Thread(target=get, args=(data_queue,)).start()

if __name__ == '__main__':
    test()
    