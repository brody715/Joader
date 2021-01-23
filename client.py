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
seq = encode({name:idx})
s.send(seq)

import os
time.sleep(1)
path = "/tmp/"+name
print(path)


f = os.open(path, os.O_RDONLY)
idx = 0

at = AvgTime()

while True:
    idx += 1
    now = time.time()
    
    if idx > n:
        s.send(encode({name:-1}))
        break
    
    size = int.from_bytes(os.read(f, 4), byteorder='big')
    stream = os.read(f, size)
    data = decode(stream)
    at.add(time.time()-now)
print(" time %f"%(at.avg()))
