from multiprocessing import shared_memory
shm = shared_memory.SharedMemory("xiejian")
shm.close()
shm.unlink()