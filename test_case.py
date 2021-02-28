import  socket
from encode import *
import time
import os
import numpy as np
ADDRESS = ('127.0.0.1', 8712) 
import threading
from buffer import Buffer

class AvgTime():
    def __init__(self):
        self.sum = 0
        self.cnt = 0
    def add(self, t):
        self.sum += t
        self.cnt += 1
    def avg(self):
        return self.sum/self.cnt


def create_socket():
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect(ADDRESS)
    return s

def send_idx(name, n, s):
    idx = list(range(n))
    size_byte, data_byte = encode((name,0,idx))
    s.send(size_byte)
    s.send(data_byte)

    size_byte = s.recv(SIZE_CNT)
    size = decode_size(size_byte)
    resp_byte = s.recv(size)
    resp = decode_data(resp_byte)
    return resp

def recv_data(s, name, bufname, n, head):
    at = AvgTime()
    print(bufname)
    
    buf = Buffer(bufname)
    idx = 0
    node = head
    s.settimeout(0)
    while idx < n:
        now = time.time()
        if idx%128 == 0:
            size_byte, data_byte = encode((name,2))
            s.send(size_byte)
            s.send(data_byte)

        next_node = buf.get_next(node)
        print(node)
        while next_node == -1:
            time.sleep(0.001)
            next_node = buf.get_next(node)
        node = next_node
        # print("read", len(buf.read(node)), time.time()-now)
        # print(node)
        at.add(time.time()-now)
        idx += 1

    print("[%s,%d]avg time %f"%(name, n, at.avg()))


def restore(name, s):
    size_byte, data_byte = encode((name, 1))
    s.send(size_byte)
    s.send(data_byte)
    resp = s.recv(1)

    return resp

def delete(name, s):
    size_byte, data_byte = encode((name,-1))
    s.send(size_byte)
    s.send(data_byte)
    resp = s.recv(1)
    return resp

def test(name, n):
    s = create_socket()
    head, buf_name = send_idx(name, n, s)
    assert(head != -1)
    recv_data(s, name, buf_name, n, head)
test("task1", 100)

# test_create_restore_del("xiejian", 100)
# test_expired("xiejian", 100)
# n = 5
# name_list = []
# n_list = []
# for i in range(n):
#     name_list.append("xiejian-GlobalLoader"+str(time.time()))
#     n_list.append(1000)
# test_multi(name_list, n_list)




