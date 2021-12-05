import sys

import grpc
sys.path.append("/home/xiej/ATC/DLCache/client/proto")

import proto.dataloader_pb2 as dataloader_pb2
import proto.dataloader_pb2_grpc as dataloader_pb2_grpc
from loader.shm import SharedMemory



class Loader(object):
    def __init__(self, ip, length: int, loader_id: int, shm_path: str):
        self.length = length
        self.client = None
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
        self.ip = ip
        self.ready = False

    @staticmethod
    def new(dataset_name:str, ip, channel=None):
        if channel is None:
            channel = grpc.insecure_channel(ip)
        client = dataloader_pb2_grpc.DataLoaderSvcStub(channel)
        request = dataloader_pb2.CreateDataloaderRequest(name=dataset_name)
        resp = client.CreateDataloader(request)
        # close to enable multi process grpc
        channel.close()
        return Loader(ip, resp.length, resp.loader_id, resp.shm_path)


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

    def read(self):
        address = self.cached_addr.pop()*self.HEAD_SIZE
        # return self.dummy_read(address*self.HEAD_SIZE)
        return self.read_data(address)

    def next(self):
        assert self.length > 0
        if not self.ready:
            self.channel = grpc.insecure_channel('127.0.0.1:4321')
            self.client = dataloader_pb2_grpc.DataLoaderSvcStub(self.channel)
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
