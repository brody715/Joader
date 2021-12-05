import ctypes
import sys
sys.path.append("./proto")

import proto.dataloader_pb2 as dataloader_pb2
import proto.dataloader_pb2_grpc as dataloader_pb2_grpc
import itertools
from loader.shm import SharedMemory



class Loader(object):
    def __init__(self, dataset_name: str, len: int, channel):
        client = dataloader_pb2_grpc.DataLoaderSvcStub(channel)
        request = dataloader_pb2.CreateDataloaderRequest(name=dataset_name)
        resp = client.CreateDataloader(request)
        self.len = len
        self.client = client
        self.loader_id = resp.loader_id
        self.shm_path = resp.shm_path
        self.shm = SharedMemory(self.shm_path)
        self.buf = self.shm.buf
        self.HEAD_SIZE = 16
        self.END = 0
        self.READ = 1
        self.LEN = 4
        self.OFF = 8

    def len(self):
        return self.len

    def read_header(self, address):
        end = self.buf[address+self.END] == 1
        len = int.from_bytes(
            self.buf[address+self.LEN:address+self.OFF], 'big')
        v = []
        v.extend(self.buf[address+self.OFF:self.HEAD_SIZE])
        off = int.from_bytes(
            self.buf[address+self.OFF:address+self.HEAD_SIZE], 'big')
        
        return end, off, len

    def read_data(self, address):
        end, off, len = self.read_header(address)
        assert end == True
        self.buf[address+self.READ] = 0
        return self.buf[off: off+len]

    def dummy_read(self, address):
        self.buf[address+self.READ] = 0
        return address

    def read(self, address):
        # return self.dummy_read(address*self.HEAD_SIZE)
        return self.read_data(address*self.HEAD_SIZE)

    def next(self):
        assert self.len > 0
        self.len -= 1
        request = dataloader_pb2.NextRequest(loader_id=self.loader_id)
        resp = self.client.Next(request)
        return self.read(resp.address)

    def delete(self):
        request = dataloader_pb2.DeleteDataloaderRequest(
            loader_id=self.loader_id)
        self.shm.close()
        resp = self.client.DeleteDataloader(request)
        return resp
