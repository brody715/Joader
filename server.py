import socket
import sampler
import numpy
from multiprocessing import Process, Queue, Manager, Pipe
import time
import loader
from encode import *
import threading
import signal, os, sys
from mylog import *

ADDRESS = ('127.0.0.1', 8712)  # 绑定地址

# 终止进程信号处理
def _signal_handler(signum, frame):
    sys.exit(0)

def init(ip, port):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.bind((ip, port))
    s.listen(5)
    return s

def message_handle(client, task_queue, task_lock, response_queue):
    while True:
        try:
            size_byte = client.recv(SIZE_CNT)
            if(len(size_byte) == 0):
                break
            
            size = decode_size(size_byte)

            packet_size = 1024
            task_byte = b''
            while size > packet_size:
                data = client.recv(packet_size)
                task_byte += data
                size -= packet_size
            task_byte += client.recv(size)
            
            task = decode_data(task_byte)

            name = task[0]
            
            with task_lock:
                while not response_queue.empty():
                    response_queue.get_nowait()
                task_queue.put(task)
                res_name, resp = response_queue.get(True)
                assert(res_name == name)
            size_byte, resp_byte = encode(resp)
            client.send(size_byte)
            client.send(resp_byte)
        except:
            return

def accept_client(s, task_queue, task_lock, response_queue):
    while True:
        try:
            client, addr = s.accept()
            logging.info("server accept a client: %s", addr)
            thread = threading.Thread(target=message_handle, args=(client, task_queue, task_lock, response_queue))
            thread.start()
        except:
            break

def start_sampler(task_queue, response_queue):
    p = Process(target=sampler.Sampler.sampler, args=(task_queue, response_queue))
    p.start()
    assert(p.is_alive() == True)
    return p

def stop_process(p):
    p.terminate()
    while p.is_alive == True:
        time.sleep(0.1)
    # assert(p.is_alive() != True)
    p.close()

if __name__ == '__main__':
    signal.signal(signal.SIGINT, _signal_handler)
    # start a Sampler process
    task_queue = Queue()
    response_queue = Queue()
    task_lock = threading.Lock()

    sampler = start_sampler(task_queue, response_queue)
    
    # start server to listen socket
    s = init(ADDRESS[0], ADDRESS[1])
    accept_client(s, task_queue, task_lock, response_queue)

    s.close()
    stop_process(sampler)
    print("---------- exited ----------")
