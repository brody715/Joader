from dataset.dataset import Dataset, DatasetType
from loader.loader import Loader
import grpc
import time

channel = grpc.insecure_channel('127.0.0.1:4321')
name = "ImageNet"
len = 100000
if __name__ == "__main__":
    ds = Dataset(name=name, location="", ty=DatasetType.DUMMY)
    now = time.time()
    for i in range(0, len):
        if i!= 0 and i % 1000 == 0:
            print("readed {} data in {} avg: {}".format(i, time.time() - now, (time.time() - now)/i))
        ds.add_item([str(i)])
    ds.create(channel)

    loader = Loader(dataset_name=name, len=len, channel=channel)
    now = time.time()
    for i in range(len):
        loader.next()
    print(time.time() - now)
    loader.delete()
    ds.delete(channel)
