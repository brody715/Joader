from enum import Enum
import sys
# dataset
sys.path.append("/home/xj/proj/DM/pytorch-imagenet/Loader")
from dataset import lmdbDataset
dataset = lmdbDataset('/data/share/ImageNet/ILSVRC-train.lmdb', True)

# config
DATASIZE = len(dataset[0])
BUFFERSIZE = (2000+1)*DATASIZE
DATASET_PATH = "/home/xj/proj/DM/pytorch-imagenet/Loader/dataset.py"
DATASET_NAME = "lmdbDataset"
HEARTBEAT_INTER = 5
WORKERS = 32
ADDRESS = ('127.0.0.1', 8712)
MESSAGE_HEAD_SIZE = 4
BYTE_ORDER = 'big'
COMMAND = Enum('COMMAND', ('DELETE', 'ADD', 'HEARTBEAT'))
MMAP_FILE_PATH = "/tmp/xiejian"
QUEUE_SIZE = 16
#############################

