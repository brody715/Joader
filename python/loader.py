import time
import threading
import multiprocessing
import queue
from mylog import logging
import torch
import signal
import os
import sys
import pickle
from buffer import Buffer
import config as cfg


class Loader(object):
    def __init__(self, in_queue, out_queue):
        self.in_queue = in_queue
        self.out_queue = out_queue
        self.pool = None

    @staticmethod
    def process(in_queue, out_queue):
        torch.set_num_threads(1)
        buf = Buffer(cfg.MMAP_FILE_PATH, cfg.DATASIZE)
        dataset = cfg.dataset
        while True:
            data_id, data_addr = in_queue.get(True)
            data = dataset[data_id]
            buf.write_data(data_addr, data)
            logging.critical("loader write data %d(len=%d) in %d",
                             data_id, len(data), data_addr)
            out_queue.put((data_id, data_addr))

    def start(self):
        logging.info("start loader")
        self.pool = multiprocessing.Pool(processes=cfg.WORKERS)
        for _ in range(cfg.WORKERS):
            self.pool.apply_async(func=Loader.process, args=(
                self.in_queue, self.out_queue))
        self.pool.close()

    def terminate(self,):
        self.pool.terminate()
        self.pool.join()
