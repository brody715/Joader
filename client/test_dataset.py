from dataset.dataset import Dataset, DatasetType
import grpc


channel = grpc.insecure_channel('210.28.134.91:4321')
name = "DUMMY"
if __name__ == "__main__":
    ds = Dataset(name=name, ty=DatasetType.DUMMY, location="")
    for i in range(0, 100):
        ds.add_item([str(i)])
    ds.create(channel)
    ds.delete(channel)
