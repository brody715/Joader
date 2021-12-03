from dataset.dataset import Dataset, DatasetType
from loader.loader import Loader
import grpc


channel = grpc.insecure_channel('127.0.0.1:4321')
name = "ImageNet"
len = 100
if __name__ == "__main__":
    ds = Dataset(name=name, ty=DatasetType.DUMMY)
    for i in range(0, len):
        ds.add_item([str(i)])
    ds.create(channel)

    loader = Loader(dataset_name=name, len=len, channel=channel)
    for i in range(len):
        print(loader.next())

    loader.delete()
    ds.delete(channel)
