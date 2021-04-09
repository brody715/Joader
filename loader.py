import loader
import dataset
import time
import multiprocessing
import queue
import threading
from mylog import *
import torch
import signal, os, sys
from buffer import Buffer

#TODO：这个需要手动定义，有点丑陋
ds = dataset.lmdbDataset('/data/share/ImageNet/ILSVRC-train.lmdb', True)

class Loader(object):
    @staticmethod
    def process(id_queue, resp_queue, buf_name):
        torch.set_num_threads(1)
        buf = Buffer(buf_name, 602116)
        while True:
            data_id, data_idx = id_queue.get(True)
            data = ds[data_id]

            buf.write_data(data_idx, data)
            logging.critical("loader write data %d(len=%d) in %d", data_id, len(data), data_idx)
            resp_queue.put((data_id, data_idx))

    @staticmethod
    #TODO: hard code workers
    def loading(id_queue, resp_queue, buf_name, workers=8, s=0):
        logging.info("start loader")
        if workers == 0:
            workers = multiprocessing.cpu_count()
        pool = multiprocessing.Pool(processes = workers)
        try:
            for i in range(workers):
                pool.apply_async(Loader.process, (id_queue, resp_queue, buf_name, ))
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
        idx_queue.put((0, 0))
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
        target=Loader.loading, args=(id_queue, data_queue, "xiejian"))
    loader.start()

    threading.Thread(target=put, args=(id_queue,)).start()
    threading.Thread(target=get, args=(data_queue,)).start()

if __name__ == '__main__':
    test()
    