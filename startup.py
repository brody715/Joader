import signal
import time
import sys
import os
import queue
import threading
from enum import Enum
from mylog import logging
from task_manger import TaskManager
from loader import Loader
from multiprocessing import Manager, Queue, Process
from buffer_manager import BufferManger
from sampling_tree_manager import SamplerTreeManager
import config as cfg


# start buffer pool manager
print("start buffer pool manager")
manager = Manager()
bm_in_queue = manager.Queue(cfg.QUEUE_SIZE)
bm_out_queue = manager.Queue(cfg.QUEUE_SIZE)
bm = BufferManger(bm_in_queue, bm_out_queue)

# start loader
print("start loader")
loader_in_queue = bm_out_queue
loader_out_queue = bm_in_queue
loader = Loader(loader_in_queue, loader_out_queue)
loader_t = threading.Thread(target=loader.start, daemon=True)
loader_t.start()

# start sampling tree manager
print("start sampling tree manager")
stm_in_queue = Queue(cfg.QUEUE_SIZE)
stm_out_queue = Queue(cfg.QUEUE_SIZE)
stm = SamplerTreeManager(stm_in_queue, stm_out_queue, cfg.MMAP_FILE_PATH, cfg.COMMAND, bm)
stm_t = Process(target=stm.start)
stm_t.start()

# start task manager
print("start task manager")
tm_out_queue = stm_in_queue
tm_in_queue = stm_out_queue
tm = TaskManager(cfg.ADDRESS[0], cfg.ADDRESS[1], cfg.BYTE_ORDER, cfg.MESSAGE_HEAD_SIZE,
                 tm_out_queue, tm_in_queue, cfg.HEARTBEAT_INTER, cfg.COMMAND)
tm_t = threading.Thread(target=tm.start, daemon=True)
tm_t.start()



# register int handler
def _signal_handler(signum, frame):
    manager.shutdown()
    stm_t.terminate()
    stm_t.join()
    loader.terminate()
    print("------------- exit -------------")
    os._exit(0)

signal.signal(signal.SIGINT, _signal_handler)

tm_t.join()
stm_t.join()



