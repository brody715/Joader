import grpc
from dataset import gen_dataset
import proto.dataset_pb2 as dataset_pb2
import proto.dataset_pb2_grpc as dataset_pb2_grpc

channel = grpc.insecure_channel('127.0.0.1:4321')
dataset_len = 10
name = "ImageNet"

# test dataset
def test_create_dataset():
    client = dataset_pb2_grpc.DatasetSvcStub(channel)
    request = dataset_pb2.CreateDatasetRequest(
        name,
        type=dataset_pb2.CreateDatasetRequest.FILESYSTEM,
        items=gen_dataset(dataset_len),
        weights=[])
    resp = client.CreateDataset(request)
    print(resp)



def test_delete_dataset():
    client = dataset_pb2_grpc.DatasetSvcStub(channel)
    request = dataset_pb2.DeleteDatasetRequest(name)
    resp = client.CreateDataset(request)
    print(resp)

if __name__ == "main":
    test_create_dataset()
    test_delete_dataset()