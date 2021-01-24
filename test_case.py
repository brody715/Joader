import  socket
from encode import *
import time
import os
import numpy as np
ADDRESS = ('127.0.0.1', 8712) 

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

def send_idx(name, n):
    idx = list(range(n))
    size_byte, data_byte = encode({name:idx})
    s.send(size_byte)
    s.send(data_byte)
    resp = s.recv(1)
    print(resp)

def rev_data(name, n):
    path = "/tmp/"+name
    print(path)
    f = os.open(path, os.O_RDONLY)
    idx = 0
    at = AvgTime()
    
    while idx <= n:
        idx += 1
        
        now = time.time()
        size_byte = os.read(f, 4)
        size = decode_size(size_byte)
        data_byte = os.read(f, size)
        data = decode_data(data_byte)
        at.add(time.time()-now)
    
    print("avg time %f"%(at.avg()))







