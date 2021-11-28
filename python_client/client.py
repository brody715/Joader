import grpc
import dataset_pb2
import dataset_pb2_grpc

channel = grpc.insecure_channel('127.0.0.1:4321')


# test dataset
def test_dataset():
    client = dataset_pb2_grpc.DatasetSvcStub(channel)
    dataitem = dataset_pb2.DataItem(keys=["1"])
    request = dataset_pb2.CreateDatasetRequest(
        name="ImageNet",
        type=dataset_pb2.CreateDatasetRequest.LMDB,
        items=[dataitem],
        weights=[1])
    resp = client.CreateDataset(request)
    print(resp)

    request = dataset_pb2.DeleteDatasetRequest(name="ImageNet")
    resp = client.DeleteDataset(request)
    print(resp)


test_dataset()