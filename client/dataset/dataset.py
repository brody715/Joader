import sys
sys.path.append("./proto")

from enum import Enum
import proto.dataset_pb2_grpc as dataset_pb2_grpc
import proto.dataset_pb2 as dataset_pb2



class DatasetType(Enum):
    FILESYSTEM = 0,
    DUMMY = 1,
    LMDB = 2,


class Dataset(object):
    type: DatasetType
    name: str
    items: list

    def __init__(self, name: str, ty: DatasetType):
        self.name = name
        self.type = ty
        self.items = []

    def add_item(self, item: list):
        self.items.append(dataset_pb2.DataItem(keys=item))

    def create(self, channel):
        client = dataset_pb2_grpc.DatasetSvcStub(channel)
        request = dataset_pb2.CreateDatasetRequest(
            name=self.name,
            type=dataset_pb2.CreateDatasetRequest.FILESYSTEM,
            items=self.items,
            weights=[])
        return client.CreateDataset(request)

    def delete(self, channel):
        client = dataset_pb2_grpc.DatasetSvcStub(channel)
        request = dataset_pb2.DeleteDatasetRequest(name=self.name)
        return client.DeleteDataset(request)

    def __len__(self):
        return len(self.items)
