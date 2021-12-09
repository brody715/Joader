from dataset.dataset import Dataset, DatasetType
from loader.loader import Loader
import time
import grpc
import lmdb
import msgpack
import random

location = "/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb"
env = lmdb.open(location,subdir=False,max_readers=100,readonly=True,lock=False,readahead=False,meminit=False)
txn = env.begin(write=False)
lmdb_len = txn.stat()['entries']
channel = grpc.insecure_channel('127.0.0.1:4321')
name = "ImageNet"
LOSE_KEY = 1281167
keys = []
for i in range(lmdb_len):
    if i != LOSE_KEY:
        keys.append(str(i))

def test_local_lmdb():
    now = time.time()
    for i, k in enumerate(keys):
        data = txn.get(k.encode())
        if i!= 0 and i % 1000 == 0:
            msgpack.loads(data, raw=False)
            print("readed {} data in {} avg: {}".format(i, time.time() - now, (time.time() - now)/i))
        

def test_global_lmdb():
    ds = Dataset(name=name, location=location, ty=DatasetType.LMDB)
    for k in keys:
        ds.add_item([k])
    ds.create(channel)

    loader = Loader.new(dataset_name=name,
                        name="lmdb_loader", ip='127.0.0.1:4321')
    now = time.time()
    for i in range(len(keys)):
        if i!= 0 and i % 1000 == 0:
            print("readed {} data in {} avg: {}".format(i, time.time() - now, (time.time() - now)/i))
        data = loader.next()
        msgpack.loads(data, raw=False)
    print(time.time() - now)
    loader.delete()
    ds.delete(channel)

if __name__ == "__main__":
    # test_local_lmdb()
    test_global_lmdb()

