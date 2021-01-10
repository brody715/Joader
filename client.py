import  socket
from encode import *
import time
import os

ADDRESS = ('127.0.0.1', 8712) 

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect(ADDRESS)



name = "xiejian"
idx = list(range(2))
seq = encode({name:idx})
s.send(seq)

import os
time.sleep(5)
path = "/tmp/"+name
print(path)
f = os.open(path, os.O_RDONLY)
s = os.read(f, 1024*1024)
data = decode(s)
print("read from "+path,":",data[0].shape, data[1])

# data = [b'e', 1]
# seq = encode(data)
# s.send(seq)