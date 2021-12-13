from dataset.dataset import Dataset, DatasetType
from loader.loader import Loader
import grpc
import time

channel = grpc.insecure_channel('210.28.134.91:4321')
name = "DummyDataset"
len = 1000
if __name__ == "__main__":
    ds = Dataset(name=name, location="", ty=DatasetType.DUMMY)
    now = time.time()
    for i in range(0, len):
        ds.add_item([str(i)])
    ds.create(channel)

    loader = Loader.new(dataset_name=name,
                        name="dummy_loader", ip='210.28.134.91:4321')
    now = time.time()
    for i in range(len):
        if i != 0 and i % 1000 == 0:
            print("readed {} data in {} avg: {}".format(
                i, time.time() - now, (time.time() - now)/i))
        loader.next()
    print(time.time() - now)
    loader.delete()
    ds.delete(channel)
