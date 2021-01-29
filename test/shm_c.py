from multiprocessing import shared_memory

shm = shared_memory.SharedMemory(name="xiejian", create=False, size=10)
buffer = shm.buf
print(buffer)
time.sleep(60)
shm.close()
shm.unlink()