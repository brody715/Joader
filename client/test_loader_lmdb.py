from dataset.dataset import Dataset, DatasetType
from loader.loader import Loader
import time
import grpc
import lmdb

location = "/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb"
env = lmdb.open(location,subdir=False,max_readers=100,readonly=True,lock=False,readahead=False,meminit=False)
txn = env.begin(write=False)
len = txn.stat()['entries']
channel = grpc.insecure_channel('127.0.0.1:4321')
name = "ImageNet"

len = 100
if __name__ == "__main__":
    ds = Dataset(name=name, location=location, ty=DatasetType.LMDB)
    for i in range(0, len):
        ds.add_item([str(i)])
    ds.create(channel)

    loader = Loader(dataset_name=name, len=len, channel=channel)
    now = time.time()
    for i in range(len):
        loader.next()
    print(time.time() - now)
    loader.delete()
    ds.delete(channel)

