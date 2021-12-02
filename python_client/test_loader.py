import grpc
from loader import *
from dataset import *
channel = grpc.insecure_channel('127.0.0.1:4321')
name = "ImageNet"

if __name__ == "__main__":
    create_dataset(channel, name)
    loader_id1 = create_loader(channel, name)
    loader_id2 = create_loader(channel, name)
    data1 = load_data(channel, loader_id1, len)
    data2 = load_data(channel, loader_id2, len)
    print(data1.sort(), data2.sort())
    delete_loader(channel, loader_id1)
    delete_loader(channel, loader_id2)
    delete_dataset(channel, name)
