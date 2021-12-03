from dataset.dataset import Dataset, DatasetType
import grpc


channel = grpc.insecure_channel('127.0.0.1:4321')
name = "ImageNet"
if __name__ == "__main__":
    ds = Dataset(name=name, ty=DatasetType.DUMMY)
    for i in range(0, 100):
        ds.add_item([str(i)])
    ds.create(channel)
    ds.delete(channel)
