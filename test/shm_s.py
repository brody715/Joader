from multiprocessing import shared_memory
import time
l = 3
shm = shared_memory.SharedMemory(name="xiejian", create=True, size=l+1)
buf = shm.buf
now = time.time()

bts = b'a'*l
buf[1:l+1] = bts

bt = buf[1:l+1].tobytes()
buf[1:l+1] = b'b'*l

print(bt, buf[1:l+1].tobytes())

print(time.time()-now)
shm.close()
shm.unlink()