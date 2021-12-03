import sys

sys.path.append("./proto")
import proto.dataset_pb2 as dataset_pb2
import proto.dataset_pb2_grpc as dataset_pb2_grpc

len = 10000


def gen_dataset(len: int):
    items = []
    for i in range(len):
        dataitem = dataset_pb2.DataItem(keys=[str(i)])
        items.append(dataitem)
    return items


def create_dataset(channel, name):
    client = dataset_pb2_grpc.DatasetSvcStub(channel)
    request = dataset_pb2.CreateDatasetRequest(
        name=name,
        type=dataset_pb2.CreateDatasetRequest.FILESYSTEM,
        items=gen_dataset(len),
        weights=[])
    resp = client.CreateDataset(request)
    print(resp)


def delete_dataset(channel, name):
    client = dataset_pb2_grpc.DatasetSvcStub(channel)
    request = dataset_pb2.DeleteDatasetRequest(name=name)
    resp = client.DeleteDataset(request)
    print(resp)