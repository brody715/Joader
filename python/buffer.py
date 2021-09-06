import signal
import os
import sys
import mmap
import config as cfg


class Buffer(object):
    def __init__(self, buffer_path, data_size, create=False, size=0):
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
        self.ADDR_SIZE = 4
        self.VALID_LEN = 3
        self.INODE_LEN = self.VALID_LEN + 2*self.ADDR_SIZE

        self.VALID_OFF = 0
        self.DATA_OFF = self.VALID_OFF + 0
        self.NEXT_OFF = self.VALID_OFF + 1
        self.USED_OFF = self.VALID_OFF + 2

        self.DATA_ADDR_OFF = self.VALID_LEN
        self.NEXT_ADDR_OFF = self.VALID_LEN + self.ADDR_SIZE

        # data = | head | data |
        self.HEADSIZE = 4
        self.BYTE_ORDER = cfg.BYTE_ORDER
        self.DATASIZE = data_size

    def read(self, inode_addr):
        data_addr_bytes = self.buf[inode_addr + self.DATA_ADDR_OFF:inode_addr +
                                   self.DATA_ADDR_OFF + self.ADDR_SIZE]
        data_addr = int.from_bytes(data_addr_bytes, self.BYTE_ORDER)

        datasize_byte = self.buf[data_addr:data_addr + self.HEADSIZE]
        datasize = int.from_bytes(datasize_byte, self.BYTE_ORDER)

        data_byte = self.buf[data_addr + self.HEADSIZE:data_addr +
                             self.HEADSIZE + datasize]

        self.buf[inode_addr+self.DATA_OFF] = 0
        return data_byte

    def parse_inode(self, inode_addr):
        data_addr_byte = self.buf[inode_addr + self.DATA_ADDR_OFF:inode_addr +
                                  self.DATA_ADDR_OFF + self.ADDR_SIZE]
        data_addr = int.from_bytes(data_addr_byte, self.BYTE_ORDER)
        next_addr_byte = self.buf[inode_addr + self.NEXT_ADDR_OFF:inode_addr +
                                  self.NEXT_ADDR_OFF + self.ADDR_SIZE]
        next_addr = int.from_bytes(next_addr_byte, self.BYTE_ORDER)
        return data_addr, next_addr

    def get_next(self, inode_addr):
        if (self.buf[inode_addr + self.NEXT_OFF]) == 0 or (self.buf[inode_addr + self.USED_OFF]) == 0:
            return -1
        next_addr_byte = self.buf[inode_addr + self.NEXT_ADDR_OFF:inode_addr +
                                  self.NEXT_ADDR_OFF + self.ADDR_SIZE]
        next_addr = int.from_bytes(next_addr_byte, self.BYTE_ORDER)
        self.buf[inode_addr + self.USED_OFF] = 0
        return next_addr

    def is_used(self, inode_addr):
        return self.buf[inode_addr+self.USED_OFF] == 1

    def is_datavalid(self, inode_addr, data_addr):
        _data_addr = int.from_bytes(
            self.buf[inode_addr + self.DATA_ADDR_OFF:inode_addr +
                     self.DATA_ADDR_OFF + self.ADDR_SIZE],
            self.BYTE_ORDER)
        return self.buf[inode_addr + self.DATA_OFF] != 0 and _data_addr == data_addr

    def allocate_inode(self):
        if self.inode_tail + self.INODE_LEN < self.data_head:
            inode_addr = self.inode_tail
            self.inode_tail += self.INODE_LEN
            self.buf[inode_addr+self.USED_OFF] = 1
            self.buf[inode_addr+self.DATA_OFF] = 0
            self.buf[inode_addr+self.NEXT_OFF] = 0
            return inode_addr
        return -1

    def allocate_datanode(self):
        if self.data_head - self.DATASIZE - self.HEADSIZE > self.inode_tail:
            self.data_head = self.data_head - self.DATASIZE - self.HEADSIZE
            return self.data_head
        return -1

    def write_inode(self, curnode_addr, lastnode_addr=-1, data_addr=-1):
        self.buf[curnode_addr+self.USED_OFF] = 1
        self.buf[curnode_addr+self.DATA_OFF] = 0
        self.buf[curnode_addr+self.NEXT_OFF] = 0

        if data_addr != -1:
            data_idx_byte = data_addr.to_bytes(self.ADDR_SIZE, self.BYTE_ORDER)

            # copy this data idx
            self.buf[curnode_addr + self.DATA_ADDR_OFF:curnode_addr +
                     self.DATA_ADDR_OFF + self.ADDR_SIZE] = data_idx_byte
            self.buf[curnode_addr + self.DATA_OFF] = 1

        # link last idx
        if lastnode_addr != -1:
            curnode_idx_byte = curnode_addr.to_bytes(self.ADDR_SIZE,
                                                     self.BYTE_ORDER)
            self.buf[lastnode_addr + self.NEXT_ADDR_OFF:lastnode_addr +
                     self.NEXT_ADDR_OFF + self.ADDR_SIZE] = curnode_idx_byte
            self.buf[lastnode_addr + self.NEXT_OFF] = 1

        return curnode_addr

    def write_data(self, data_addr, data):
        assert(len(data) == self.DATASIZE)
        size_byte = len(data).to_bytes(self.HEADSIZE,
                                       byteorder=self.BYTE_ORDER)
        # write data
        self.buf[data_addr:data_addr+self.HEADSIZE +
                 self.DATASIZE] = size_byte + data
        return data_addr

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
