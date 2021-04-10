import signal, os, sys
sys.path.append("/home/xj/proj/DM/pytorch-imagenet/Loader/")
import mmap



class Buffer(object):
    def __init__(self, buffer_path, data_len, create=False, size=0):
        if create:
            assert(size != 0)
            f = open(buffer_path, "wb+")
            f.write(b'0'*size)
            self.buf = mmap.mmap(f.fileno(), size)
            self.buf.flush()
        else:
            f = open(buffer_path, "rb+")
            self.buf = mmap.mmap(f.fileno(), size)

        self.create = create
        self.size = size
        self.buffer_path = buffer_path

        # | -- inode -->     <-- data --|
        self.inode_tail = 0
        self.data_head = size


        # basic config
        # | VALID BYTE | DATA_IDX | NEXT_IDX |
        self.INDEX_LEN = 4
        self.VALID_LEN = 3
        self.INODE_LEN = self.VALID_LEN + 2*self.INDEX_LEN

        self.VALID_OFF = 0
        self.DATA_OFF = self.VALID_OFF + 0
        self.NEXT_OFF = self.VALID_OFF + 1
        self.USED_OFF = self.VALID_OFF + 2

        self.DATA_IDX_OFF = self.VALID_LEN
        self.NEXT_IDX_OFF = self.VALID_LEN + self.INDEX_LEN

        # data = | datasize | data |
        self.DATASIZE_LEN = 4
        self.BYTE_ORDER = 'big'
        self.DATA_LEN = data_len
    
    def print_inode(self, idx):
        data_idx_byte = self.buf[idx + self.DATA_IDX_OFF:idx +
                                 self.DATA_IDX_OFF + self.INDEX_LEN]
        data_idx = int.from_bytes(data_idx_byte, self.BYTE_ORDER)
        next_idx_byte = self.buf[idx + self.NEXT_IDX_OFF:idx +
                                 self.NEXT_IDX_OFF + self.INDEX_LEN]
        next_idx = int.from_bytes(next_idx_byte, self.BYTE_ORDER)
        # print(idx, ":", self.buf[idx], data_idx, next_idx)

    def read(self, idx):
        data_idx_byte = self.buf[idx + self.DATA_IDX_OFF:idx +
                                 self.DATA_IDX_OFF + self.INDEX_LEN]
        data_idx = int.from_bytes(data_idx_byte, self.BYTE_ORDER)

        datasize_byte = self.buf[data_idx:data_idx + self.DATASIZE_LEN]
        datasize = int.from_bytes(datasize_byte, self.BYTE_ORDER)

        data_byte = self.buf[data_idx + self.DATASIZE_LEN:data_idx +
                             self.DATASIZE_LEN + datasize]

        self.buf[idx + self.DATA_OFF] = 0
        return data_byte
    
    def parse_inode(self, idx):
        data_idx_byte = self.buf[idx + self.DATA_IDX_OFF:idx +
                                 self.DATA_IDX_OFF + self.INDEX_LEN]
        data_idx = int.from_bytes(data_idx_byte, self.BYTE_ORDER)
        next_idx_byte = self.buf[idx + self.NEXT_IDX_OFF:idx +
                                 self.NEXT_IDX_OFF + self.INDEX_LEN]
        next_idx = int.from_bytes(next_idx_byte, self.BYTE_ORDER)
        return data_idx, next_idx
    
    def get_next(self, idx):
        if (self.buf[idx + self.NEXT_OFF]) == 0 or (self.buf[idx + self.USED_OFF]) == 0:
            return -1
        next_idx_byte = self.buf[idx + self.NEXT_IDX_OFF:idx +
                                 self.NEXT_IDX_OFF + self.INDEX_LEN]
        next_idx = int.from_bytes(next_idx_byte, self.BYTE_ORDER)
        self.buf[idx + self.USED_OFF] = 0
        return next_idx

    def is_used(self, inode_idx):
        return self.buf[inode_idx+self.USED_OFF] == 1

    def is_datavalid(self, inode_idx, data_idx):
        _data_idx = int.from_bytes(
                        self.buf[inode_idx + self.DATA_IDX_OFF:inode_idx +
                                 self.DATA_IDX_OFF + self.INDEX_LEN],
                        self.BYTE_ORDER)
        return self.buf[inode_idx + self.DATA_OFF] != 0 and _data_idx == data_idx

    def allocate_inode(self):
        if self.inode_tail + self.INODE_LEN < self.data_head:
            idx = self.inode_tail
            self.inode_tail += self.INODE_LEN
            self.buf[idx+self.DATA_OFF] = 0
            self.buf[idx+self.USED_OFF] = 0
            self.buf[idx+self.NEXT_OFF] = 0
            return idx
        return -1

    def allocate_datanode(self):
        if self.data_head - self.DATA_LEN - self.DATASIZE_LEN > self.inode_tail:
            self.data_head = self.data_head - self.DATA_LEN - self.DATASIZE_LEN
            return self.data_head
        return -1

    def write_inode(self, curnode_idx, lastnode_idx=-1, data_idx=-1):
        self.buf[curnode_idx+self.USED_OFF] = 1
        self.buf[curnode_idx+self.DATA_OFF] = 0
        self.buf[curnode_idx+self.NEXT_OFF] = 0
    
        if data_idx != -1:
            data_idx_byte = data_idx.to_bytes(self.INDEX_LEN, self.BYTE_ORDER)

            # copy this data idx
            self.buf[curnode_idx + self.DATA_IDX_OFF:curnode_idx +
                    self.DATA_IDX_OFF + self.INDEX_LEN] = data_idx_byte
            self.buf[curnode_idx + self.DATA_OFF] = 1

        # link last idx
        if lastnode_idx != -1:
            curnode_idx_byte = curnode_idx.to_bytes(self.INDEX_LEN,
                                                        self.BYTE_ORDER)
            self.buf[lastnode_idx + self.NEXT_IDX_OFF:lastnode_idx +
                    self.NEXT_IDX_OFF + self.INDEX_LEN] = curnode_idx_byte
            self.buf[lastnode_idx + self.NEXT_OFF] = 1

        return curnode_idx

    def write_data(self, data_idx, data):
        assert(len(data) == self.DATA_LEN)
        size_byte = len(data).to_bytes(self.DATASIZE_LEN,
                                       byteorder=self.BYTE_ORDER)
        # write data
        self.buf[data_idx:data_idx+self.DATA_LEN+self.DATASIZE_LEN] = size_byte + data
        return data_idx

    def debug_print(self, ):
        for i in range(0, self.inode_tail, self.INODE_LEN):
            print(self.buf[i],
                  int.from_bytes(self.buf[i + 1:i + 5], self.BYTE_ORDER),
                  int.from_bytes(self.buf[i + 5:i + 9], self.BYTE_ORDER),
                  end=' | ')

    def __del__(self, ):
        pass
        # self.shm.close()
        # self.shm.unlink()


