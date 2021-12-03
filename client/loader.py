import sys

sys.path.append("./proto")
import proto.dataloader_pb2_grpc as dataloader_pb2_grpc
import proto.dataloader_pb2 as dataloader_pb2


def create_loader(channel, name):
    client = dataloader_pb2_grpc.DataLoaderSvcStub(channel)
    request = dataloader_pb2.CreateDataloaderRequest(name=name)
    resp = client.CreateDataloader(request)
    print(resp)
    return resp.loader_id


def delete_loader(channel, loader_id):
    client = dataloader_pb2_grpc.DataLoaderSvcStub(channel)
    request = dataloader_pb2.DeleteDataloaderRequest(loader_id=loader_id)
    resp = client.DeleteDataloader(request)
    print(resp)

def load_data(channel, loader_id, len:int):
    client = dataloader_pb2_grpc.DataLoaderSvcStub(channel)
    data = []
    for _ in range(len):
        request = dataloader_pb2.NextRequest(loader_id=loader_id)
        resp = client.Next(request)
        data.append(resp.address)
    return data