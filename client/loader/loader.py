import sys

import grpc
sys.path.append("/home/xiej/ATC/DLCache/client/proto")

import proto.dataloader_pb2 as dataloader_pb2
import proto.dataloader_pb2_grpc as dataloader_pb2_grpc
import itertools
from loader.shm import SharedMemory



class Loader(object):
    def __init__(self, ip, length: int, loader_id: int, shm_path: str, client):
        self.length = length
        self.client = client
        self.loader_id = loader_id
        self.shm_path = shm_path
        self.shm = SharedMemory(self.shm_path)
        self.buf = self.shm.buf
        self.cached_addr = []

        self.HEAD_SIZE = 16
        self.END = 0
        self.READ = 1
        self.LEN = 4
        self.OFF = 8
        self.ip=ip

    @staticmethod
    def new(dataset_name:str, ip, channel=None):
        if channel is None:
            channel = grpc.insecure_channel('127.0.0.1:4321')
        client = dataloader_pb2_grpc.DataLoaderSvcStub(channel)
        request = dataloader_pb2.CreateDataloaderRequest(name=dataset_name)
        resp = client.CreateDataloader(request)
        return Loader(ip, resp.length, resp.loader_id, resp.shm_path, client)

    def clone(self):
        channel = grpc.insecure_channel('127.0.0.1:4321')
        client = dataloader_pb2_grpc.DataLoaderSvcStub(channel)
        return Loader(self.ip, self.length, self.loader_id, self.shm_path, client)

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
        print(end, off, len)
        assert end == True
        self.buf[address+self.READ] = 0
        return self.buf[off: off+len]

    def dummy_read(self, address):
        self.buf[address+self.READ] = 0
        return address

    def read(self):
        address = self.cached_addr.pop()*self.HEAD_SIZE
        # return self.dummy_read(address*self.HEAD_SIZE)
        return self.read_data(address)

    def next(self):
        assert self.length > 0
        self.length -= 1
        while len(self.cached_addr) == 0:
            request = dataloader_pb2.NextRequest(loader_id=self.loader_id)
            resp = self.client.Next(request)
            self.cached_addr = resp.address
        return self.read()

    def delete(self):
        request = dataloader_pb2.DeleteDataloaderRequest(
            loader_id=self.loader_id)
        # Todo(xj):bug
        # self.shm.close()
        resp = self.client.DeleteDataloader(request)
        return resp
