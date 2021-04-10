import time
import threading
import multiprocessing
import queue
from mylog import logging
import torch
import signal
import os
import sys
from buffer import Buffer


class Loader(object):
    def __init__(self, workers, buffer_path, in_queue, out_queue, data_size, dataset):
        self.workers = workers
        self.buffer_path = buffer_path
        self.buffer = Buffer(buffer_path, data_size)
        self.in_queue = in_queue
        self.out_queue = out_queue
        self.dataset = dataset
        self.pool = None

    def process(self, ):
        torch.set_num_threads(1)
        while True:
            data_id, data_addr = self.in_queue.get(True)
            data = self.dataset[data_id]
            self.buffer.write_data(data_addr, data)
            logging.critical("loader write data %d(len=%d) in %d",
                             data_id, len(data), data_addr)
            self.out_queue.put((data_id, data_addr))

    def start(self):
        logging.info("start loader")
        self.pool = multiprocessing.Pool(processes=self.workers)
        try:
            for _ in range(self.workers):
                self.pool.apply_async(self.process, ())
            self.pool.close()
            self.pool.join()
        except:
            logging.error("loader exit")

    def terminate(self,):
        self.pool.terminate()


n = 1000

def put(idx_queue):
    for i in range(n):
        print("put", i)
        idx_queue.put((0, 0))


def get(data_queue):
    t = time.time()
    for i in range(n):
        data_queue.get()
        print("put", i)


def test():
    import config as cfg
    in_queue = multiprocessing.Manager().Queue(maxsize=cfg.QUEUE_SIZE)
    out_queue = multiprocessing.Manager().Queue(maxsize=cfg.QUEUE_SIZE)
    loader = Loader(8, "/tmp/xiejian", in_queue, out_queue, cfg.DATASIZE, cfg.dataset)
    threading.Thread(target=put, args=(in_queue,)).start()
    threading.Thread(target=get, args=(out_queue,)).start()
    loader.start()

if __name__ == '__main__':
    test()
