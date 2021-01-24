import socket
import sampler
import numpy
from multiprocessing import Process, Queue, Manager, Pipe
import time
import loader
from encode import *
import threading
import signal, os
from mylog import *

ADDRESS = ('127.0.0.1', 8712)  # 绑定地址

# 终止进程信号处理
global_process_list = []

def init(ip, port):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.bind((ip, port))
    s.listen(5)
    return s

def message_handle(client, task_queue, task_lock):
    idx_list = []
    while True:
        size_byte = client.recv(SIZE_CNT)

        size = decode_size(size_byte)
        task_byte = client.recv(size)
        
        if (len(task_byte) == 0):
            break

        task = decode_data(task_byte)
        
        
        with task_lock:
            task_queue.put(task)
            resp = task_queue.get()
        client.send(resp)

        if list(task.values())[0] == -1 or resp == b'0':
            break

def accept_client(s, task_queue, task_lock):
    while True:
        client, addr = s.accept()
        logging.info("server accept a client: %s", addr)
        thread = threading.Thread(target=message_handle, args=(client, task_queue, task_lock))
        thread.start()

def start_sampler(task_queue, idx_queue, data_queue):
    p = Process(target=sampler.Sampler.sampler, args=(task_queue, idx_queue, data_queue))
    p.start()
    assert(p.is_alive() == True)
    return p

def start_loader(idx_queue, data_queue):
    p = Process(target=loader.Loader.loading, args=(idx_queue, data_queue))
    p.start()
    assert(p.is_alive() == True)
    return p

def stop_process(p):
    p.terminate()
    time.sleep(0.1)
    p.close()

if __name__ == '__main__':
    # start a Sampler process
    task_queue = Queue()
    task_lock = threading.Lock()

    idx_queue = Manager().Queue()
    data_queue = Manager().Queue()

    sampler = start_sampler(task_queue, idx_queue, data_queue)
    loader = start_loader(idx_queue, data_queue)
    
    # start server to listen socket
    s = init(ADDRESS[0], ADDRESS[1])
    accept_client(s, task_queue, task_lock)

    
    