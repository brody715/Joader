from multiprocessing import shared_memory

shm = shared_memory.SharedMemory(name="xiejian", create=True, size=10)
buffer = shm.buf
buffer[0:4] = [0,1,2,3]
time.sleep(60)
shm.close()
shm.unlink()