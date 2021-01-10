import reader
import dataset
import time
import multiprocessing
from multiprocessing import Process, Queue, Pool
import logging
import threading
LOG_FORMAT = "[%(asctime)s]%(levelname)s : %(message)s"
logging.basicConfig(level=logging.DEBUG, format=LOG_FORMAT)

def test(idx):
    logging.info("reader decode testing %d", idx)



class Reader(object):
    @staticmethod
    def process(item, data_queue):
        idx, data = item
        logging.info("reader process idx: %d", idx)
        data = dataset.lmdbDataset.process(data, True)
        data_queue.put((idx, data))
    
    @staticmethod
    def multi_process(ds, middle_queue, data_queue, workers=0):
        if workers == 0:
            workers = multiprocessing.cpu_count()
        print(workers)
        p = Pool(processes = workers)
        while True:
            item = middle_queue.get(True)
            p.apply_async(Reader.process, (item, data_queue, ))
            
    
    @staticmethod
    def stop_pool(mp):
        mp.close()
        mp.join()
        
    @staticmethod
    def reader(idx_queue, data_queue):
        logging.info("start reader")
        ds = dataset.lmdbDataset('/data/share/ImageNet/ILSVRC-train.lmdb', True)
        middle_queue = Queue()
        
        thred = threading.Thread(target=Reader.multi_process, args=(ds, middle_queue, data_queue))
        thred.start()

        while True:
            idx = idx_queue.get(True)
            logging.info("reader read idx: %d", idx)
            middle_queue.put((idx, ds[idx]))
            time.sleep(0.5)