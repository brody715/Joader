import grpc
import sys
sys.path.append("./proto")

from loader.shm import SharedMemory
import proto.dataloader_pb2_grpc as dataloader_pb2_grpc
import proto.dataloader_pb2 as dataloader_pb2
import socket





class Loader(object):
    def __init__(self, ip, length: int, loader_id: int, shm_path: str, name: str, dataset_name: str, server_ip: str, nums: int):
        self.length = length
        self.client = None
        self.loader_id = loader_id
        self.shm_path = shm_path
        self.shm = SharedMemory(self.shm_path)
        self.buf = self.shm.buf
        self.cached_addr = []
        self.name = name
        self.dataset_name = dataset_name
        self.channel = None
        self.server_ip = server_ip

        self.HEAD_SIZE = 20
        self.READ_OFF = 12
        self.LEN_OFF = 0
        self.OFF_OFF = 4
        self.READ_LEN = 8
        self.LEN_LEN = 4
        self.OFF_LEN = 8
        self.ip = ip
        self.nums = nums

    @staticmethod
    def new(dataset_name: str, name: str, ip: str, nums: int = 1):
        # nums indicate the number of distributed tasks
        channel = grpc.insecure_channel(ip)
        client = dataloader_pb2_grpc.DataLoaderSvcStub(channel)
        request = dataloader_pb2.CreateDataloaderRequest(
            dataset_name=dataset_name, name=name, nums=nums)
        resp = client.CreateDataloader(request)
        # close to enable multi process grpc
        channel.close()
        return Loader(ip, resp.length, resp.loader_id, resp.shm_path, name, dataset_name, ip, nums)

    def get_host_ip(self):
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        s.connect(('8.8.8.8', 80))
        ip = s.getsockname()[0]
        s.close()
        return str(ip)

    def read_header(self, address):
        len = int.from_bytes(
            self.buf[address+self.LEN_OFF:address+self.LEN_OFF+self.LEN_LEN], 'big')
        off = int.from_bytes(
            self.buf[address+self.OFF_OFF:address+self.OFF_OFF+self.OFF_LEN], 'big')
        return off, len

    def read_data(self, address, read_off):
        off, len = self.read_header(address)
        self.buf[address+self.READ_OFF + read_off] = 0
        return self.buf[off: off+len]

    def dummy_read(self, address):
        self.buf[address+self.READ] = 0
        return int(address/self.HEAD_SIZE)

    def next(self):
        assert self.length > 0
        if self.channel is None:
            self.channel = grpc.insecure_channel(self.server_ip)
            self.client = dataloader_pb2_grpc.DataLoaderSvcStub(self.channel)
        self.length -= 1
        request = dataloader_pb2.NextRequest(loader_id=self.loader_id)
        resp = self.client.Next(request)
        read_off = resp.read_off
        addr = resp.address[0]*self.HEAD_SIZE
        return self.read_data(addr, read_off)

    def delete(self):
        if self.channel is None:
            self.channel = grpc.insecure_channel(self.server_ip)
            self.client = dataloader_pb2_grpc.DataLoaderSvcStub(self.channel)
        request = dataloader_pb2.DeleteDataloaderRequest(
            dataset_name=self.dataset_name, name=self.name)
        # Todo(xj): bug
        # self.shm.close()
        resp = self.client.DeleteDataloader(request)
        return resp
    def reset(self):
        if self.channel is None:
            self.channel = grpc.insecure_channel(self.server_ip)
            self.client = dataloader_pb2_grpc.DataLoaderSvcStub(self.channel)
        request = dataloader_pb2.DeleteDataloaderRequest(
            dataset_name=self.dataset_name, name=self.name)
        # Todo(xj): bug
        # self.shm.close()
        self.client.DeleteDataloader(request)
        request = dataloader_pb2.CreateDataloaderRequest(
            dataset_name=self.dataset_name, name=self.name, nums=self.nums)
        self.client.CreateDataloader(request)