import proto.dataloader_pb2 as dataloader_pb2
import proto.dataloader_pb2_grpc as dataloader_pb2_grpc
import sys
from loader.shm import SharedMemory


sys.path.append("./proto")


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
    def len(self):
        return self.len

    def read_header(self, address):
        end = self.buf[address+0] == 1
        self.buf[address+1] = 1
        off = int.from_bytes(self.buf[address+4:address+8], 'big')
        len = int.from_bytes(self.buf[address+8:address+16], 'big')
        return end, off, len

    def read(self, address):
        print(self.read_header(address))
        return address

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
