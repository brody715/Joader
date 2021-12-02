import sys
sys.path.append("./proto")
import proto.dataset_pb2 as dataset_pb2
import proto.dataset_pb2_grpc as dataset_pb2_grpc

def gen_dataset(len: int):
    items = []
    for i in range(len):
        dataitem = dataset_pb2.DataItem(keys=[str(i)])
        items.append(dataitem)
    items