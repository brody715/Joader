import  socket
from encode import *
import time
import os
import numpy as np
from collate import *

import torch
class GlobalDataSet(torch.utils.data.IterableDataset):
    def __init__(self, address, idx_list=list(range(1000)), name="xiejian"):
        self.s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.s.connect(address)
        self.name = name
        size, seq = encode({self.name:idx_list})
        self.s.send(size)
        self.s.send(seq)
        resp = self.s.recv(1)

        assert(resp == b'1')

        time.sleep(1) # 等待文件创建完成
        self.path = "/tmp/"+name
        self.fd = os.open(self.path, os.O_RDONLY)
        
        self.length = len(idx_list)
        self.cnt = 0
    def __len__(self):
        return self.length
    
    def __iter__(self):
        return self

    def __next__(self):
        self.cnt += 1
        if self.cnt >= self.length:
            raise StopIteration
        size_byte = os.read(self.fd, SIZE_CNT)
        size = decode_size(size_byte)

        data_byte = os.read(self.fd, size)
        data = decode_data(data_byte)
        
        return data

    def reset(self):
        self.cnt = 0
        size, seq = encode({name:1})
        self.s.send(size)
        self.s.send(seq)
    
    def __del__(self):
        # os.close(self.fd)
        self.s.close()


class AvgTime():
    def __init__(self):
        self.sum = 0
        self.cnt = 0
    def add(self, t):
        if t > 0:
            self.sum += t
            self.cnt += 1
    def avg(self):
        if self.cnt == 0:
            return 0
        return self.sum/self.cnt

def test():
    ADDRESS = ('127.0.0.1', 8712)
    gd = GlobalDataSet(address=ADDRESS, idx_list = list(range(12)), name="lmdbdataset"+str(time.time()))
    avg = AvgTime()
    loader = torch.utils.data.DataLoader(gd, batch_size=4)
    now = time.time()
    for input, target in loader:
        # avg.add(time.time()-now)
        print(input.shape, time.time()-now)
        time.sleep(1)
        now = time.time()
# test()