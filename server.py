import socket
import cache
import sampler
import numpy
from multiprocessing import Process, Queue, Manager
import time
import reader
from encode import *
from threading import Thread


ADDRESS = ('127.0.0.1', 8712)  # 绑定地址

def init(ip, port):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.bind((ip, port))
    s.listen(5)
    return s

def message_handle(client, task_queue):
    idx_list = []
    while True:
        bts = client.recv(1024*1024)
        
        if (len(bts) == 0):
            break

        data = decode(bts)
        task_queue.put(data)
        print(data)

def accept_client(s, task_queue):
    while True:
        client, addr = s.accept()
        thread = Thread(target=message_handle, args=(client, task_queue))
        thread.start()

def start_sampler(task_queue, idx_queue, data_queue):
    p = Process(target=sampler.Sampler.sampler, args=(task_queue, idx_queue, data_queue))
    p.start()
    assert(p.is_alive() == True)
    return p

def start_reader(idx_queue, data_queue):
    p = Process(target=reader.Reader.reader, args=(idx_queue, data_queue))
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
    idx_queue = Queue()
    data_queue = Manager().Queue()

    start_sampler(task_queue, idx_queue, data_queue)
    start_reader(idx_queue, data_queue)
    # start server to listen socket
    s = init(ADDRESS[0], ADDRESS[1])
    accept_client(s, task_queue)
    s.close()

    stop_sampler(p)