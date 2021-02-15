import torch
import os
import cv2
import time
import numpy as np
import torchvision.transforms as transforms
import lmdb
import msgpack
from torch.utils.data import Dataset
from PIL import Image
from mylog import *
from monitor import *
class lmdbDataset(Dataset):

    def __init__(self, location, is_train):
        self.env = lmdb.open(location,subdir=False,max_readers=1,readonly=True,lock=False,readahead=False,meminit=False)
        self.txn = self.env.begin(write=False)
        self.length = self.txn.stat()['entries']
        normalize = transforms.Normalize(mean=[0.485, 0.456, 0.406],
                                         std=[0.229, 0.224, 0.225])
        #train data augment
        if is_train:
            self.transform = transforms.Compose([
                transforms.RandomResizedCrop(224),
                transforms.RandomHorizontalFlip(),
                transforms.ToTensor(),
                normalize,
            ])
        #test data augment
        else:
            self.transform = transforms.Compose([
            transforms.Resize(256),
            transforms.CenterCrop(224),
            transforms.ToTensor(),
            normalize,
        ])
        '''
        for key,data in self.txn.cursor():
            now_data = msgpack.loads(data,raw=False)
            data_img = now_data[0]
            label = now_data[1]
            now_arr = np.frombuffer(data_img[b'data'],dtype=np.uint8)
            print(now_arr)
            image_content = cv2.imdecode(now_arr, cv2.IMREAD_COLOR)
            print(image_content.shape)

            #print(type(_))
            break
        '''
    def __len__(self):
        return self.length - 1
    def __getitem__(self, index):
        begin = time.time()

        new_index = str(index).encode()
        data = self.txn.get(new_index)
        now_data = msgpack.loads(data, raw=False)
        data_img = now_data[0]
        label = now_data[1]
        now_arr = np.frombuffer(data_img[b'data'], dtype=np.uint8)
        
        # load_time = time.time()-begin
        # begin = time.time()
        
        image_content = cv2.imdecode(now_arr, cv2.IMREAD_COLOR)
        image_content = cv2.cvtColor(image_content,cv2.COLOR_BGR2RGB)
        image_content = Image.fromarray(image_content)
        image_content = self.transform(image_content)
        
        process_time = time.time()-begin
        return self.to_bytes(image_content, label)

    def to_bytes(self, image_content, label):
        image_content_bytes = np.array(image_content).tobytes()
        label_bytes = label.to_bytes(4, 'big')
        # print np.array(image_content_bytes)
        return label_bytes+image_content_bytes

    def from_bytes(self, bytes):
        label = int.from_bytes(bytes[:4], 'big')
        image_content = np.frombuffer(bytes[4:], uint8)
        return label, torch.from_numpy(image_content)




if __name__ == '__main__':
    ds = lmdbDataset('/data/share/ImageNet/ILSVRC-train.lmdb', True)
    print(len(ds[0]))
    print(len(ds[1]))
    print(len(ds[15]))
    # print(ds[0])
        #print(i)
        #assert temp_dataset[i][0] is not None
