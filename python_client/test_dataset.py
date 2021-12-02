import grpc
from dataset import *
channel = grpc.insecure_channel('127.0.0.1:4321')
name = "ImageNet"
if __name__ == "__main__":
    create_dataset(channel, name)
    delete_dataset(channel, name)