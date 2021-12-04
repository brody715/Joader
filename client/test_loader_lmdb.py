from dataset.dataset import Dataset, DatasetType
import grpc
import lmdb

location = "/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb"
env = lmdb.open(location,subdir=False,max_readers=100,readonly=True,lock=False,readahead=False,meminit=False)
txn = env.begin(write=False)
len = txn.stat()['entries']
channel = grpc.insecure_channel('127.0.0.1:4321')
name = "ImageNet"


def create_lmdb():
    pass
