import  socket
from encode import *
import time
import os
import numpy as np
ADDRESS = ('127.0.0.1', 8712) 
import threading

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
    size_byte, data_byte = encode({name:idx})
    s.send(size_byte)
    s.send(data_byte)
    resp = s.recv(1)
    
    return resp

def rev_data(name, n):
    path = "/tmp/"+name
    print(path)
    f = os.open(path, os.O_RDONLY)
    idx = 0
    at = AvgTime()
    
    while idx < n:
        now = time.time()
        size_byte = os.read(f, 4)
        size = decode_size(size_byte)
        data_byte = os.read(f, size)
        data = decode_data(data_byte)
        at.add(time.time()-now)
        idx += 1

    print("[%s,%d]avg time %f"%(name, n, at.avg()))


def restore(name, s):
    size_byte, data_byte = encode({name:1})
    s.send(size_byte)
    s.send(data_byte)
    resp = s.recv(1)

    return resp

def delete(name, s):
    size_byte, data_byte = encode({name:-1})
    s.send(size_byte)
    s.send(data_byte)
    resp = s.recv(1)

    return resp

def test_create_restore_del(name, n):
    s = create_socket()
    print("conn sucess")
    
    resp = send_idx(name, n, s)
    assert(resp == b'1')

    rev_data(name, n)

    resp = restore(name, s)
    assert(resp == b'1')

    rev_data(name, n)

    resp = delete(name, s)
    assert(resp == b'1')
    assert(not os.path.exists("/tmp/"+name))

def test_multi(name_list, n_list):
    assert(len(name_list) == len(n_list))
    
    for i in range(len(name_list)):
        t = threading.Thread(target=test_create_restore_del, args=(name_list[i], n_list[i]))
        t.start()
    time.sleep(60)

def test_expired(name, n):
    s = create_socket()
    print("conn sucess")
    
    resp = send_idx(name, n, s)
    assert(resp == b'1')

    rev_data(name, n)

    resp = send_idx(name, n, s)
    assert(resp == b'0')

    time.sleep(60)
    resp = send_idx(name, n, s)
    assert(resp == b'1')

# test_create_restore_del("xiejian", 100)
# test_expired("xiejian", 100)
n = 10
name_list = []
n_list = []
for i in range(n):
    name_list.append("xiejian-GlobalLoader"+str(time.time()))
    n_list.append(1000)
test_multi(name_list, n_list)




