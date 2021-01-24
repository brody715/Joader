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

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect(ADDRESS)

name = "xiejian"
n = 100
idx = list(range(n))
size_byte, data_byte = encode({name:idx})
s.send(size_byte)
s.send(data_byte)

resp = s.recv(1)
print(resp)

import os
path = "/tmp/"+name
print(path)


f = os.open(path, os.O_RDONLY)
idx = 0

at = AvgTime()

while True:
    idx += 1
    
    
    if idx > n:
        print("try delete")
        size_byte, data_byte = encode({name: -1})
        s.send(size_byte)
        s.send(data_byte)
        resp = s.recv(1)
        print(resp)
        if resp == b'1':
            print("delete succ")
        break
    
    now = time.time()
    size_byte = os.read(f, 4)
    size = decode_size(size_byte)
    data_byte = os.read(f, size)
    data = decode_data(data_byte)
    at.add(time.time()-now)
print(" time %f"%(at.avg()))
