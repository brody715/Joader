from os import read
import grpc
import sys
sys.path.append("./proto")

from loader.shm import SharedMemory
import proto.dataloader_pb2_grpc as dataloader_pb2_grpc
import proto.dataloader_pb2 as dataloader_pb2
import socket
import multiprocessing
import threading

class Loader(object):
    def __init__(self, ip, length: int, loader_id: int, shm_path: str, name: str, dataset_name: str, server_ip: str, nums: int, bs: int, queue: multiprocessing.Queue, client_thread):
        self.length = length
        self.client = None
        self.client_thread = client_thread
        self.loader_id = loader_id
        self.shm_path = shm_path
        self.shm = SharedMemory(self.shm_path)
        self.buf = self.shm.buf
        self.name = name
        self.dataset_name = dataset_name
        self.channel = None
        self.server_ip = server_ip
        self.bs = bs

        self.HEAD_SIZE = 20
        self.READ_OFF = 12
        self.LEN_OFF = 0
        self.OFF_OFF = 4
        self.READ_LEN = 8
        self.LEN_LEN = 4
        self.OFF_LEN = 8
        self.ip = ip
        self.nums = nums
        #profile
        self.read_time = 0
        self.rpc_time = 0
        self.queue = queue
    
    @staticmethod
    def run_client(dataset_name: str, name: str, ip: str, nums: int, batch_size, q):
        channel = grpc.insecure_channel(ip)
        client = dataloader_pb2_grpc.DataLoaderSvcStub(channel)
        request = dataloader_pb2.CreateDataloaderRequest(
            dataset_name=dataset_name, name=name, nums=nums)
        resp = client.CreateDataloader(request)
        loader_id = resp.loader_id
        length = resp.length
        q.put(resp.length)
        q.put(resp.loader_id)
        q.put(resp.shm_path)
        while length > 0:
            request = dataloader_pb2.NextRequest(loader_id=loader_id, batch_size=batch_size)
            resp = client.Next(request)
            read_off_list = resp.read_off
            address_list = resp.address
            for (read_off, address) in zip(read_off_list,address_list):
                q.put((read_off, address))
            length -= len(read_off_list)
    @staticmethod
    def new(dataset_name: str, name: str, ip: str, nums: int = 1, batch_size: int = -1):
        q = multiprocessing.Queue()
        t = threading.Thread(target=Loader.run_client, args=(dataset_name, name, ip, nums, batch_size, q,))
        t.start()
        # nums indicate the number of distributed tasks
        length = q.get()
        loader_id = q.get()
        shm_path = q.get()
        return Loader(ip, length, loader_id, shm_path, name, dataset_name, ip, nums, batch_size, q, t)

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
    
    def read_one(self):
        self.read_off, self.addr = self.queue.get()
        self.addr = self.addr*self.HEAD_SIZE
        data = self.read_data(self.addr, self.read_off)
        return data

    def next(self):
        assert self.length > 0
        self.length -= 1
        return self.read_one()

    def readed(self):
        self.buf[self.addr+self.READ_OFF + self.read_off] = 0

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