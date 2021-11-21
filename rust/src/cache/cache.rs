use core::time;
use std::slice::from_raw_parts_mut;
use std::{ptr, thread};

use crate::cache::{head_end, head_len, head_off, write_head, Buffer, DataSegment, HeadSegment};
use libc::{c_void, ftruncate, mmap, shm_open};
use libc::{off_t, shm_unlink};
use libc::{MAP_SHARED, O_CREAT, O_RDWR, PROT_WRITE, S_IRUSR, S_IWUSR};

use super::write_head_end;

pub struct Cache {
    name: String,
    capacity: usize,
    head_size: u32,
    head_segment: HeadSegment,
    data_segment: DataSegment,
    start_addr: *mut u8,
}

const HEAD_SIZE: u32 = 16;
const HEAD_NUM: u64 = 8;

#[derive(Debug, Clone, Copy)]
pub struct DataBlock {
    head: Buffer,
    data: Buffer,
    // In the data, some data willl be reserved when call next_block()
    reserve: isize,
}

impl DataBlock {
    pub fn is_valid(&self) -> bool {
        self.data.len() > self.reserve as u64
    }

    pub fn size(&self) -> u64 {
        self.data.len() - self.reserve as u64
    }

    pub fn ptr(&mut self) -> *mut c_void {
        unsafe { self.data.as_mut_ptr().offset(self.reserve).cast::<c_void>() }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe {
            from_raw_parts_mut(
                self.data.as_mut_ptr().offset(self.reserve),
                self.size() as usize,
            )
        }
    }

    pub fn occupy(&mut self, size: usize) -> Option<DataBlock> {
        if size == 0 {
            return Some(*self);
        }

        let len = self.data.len();
        // copy last head data to new data block
        self.data.copy_from_buffer(&self.head, 0);
        // write head the meta information
        write_head(
            self.head.as_mut_ptr(),
            self.data.offset(),
            size as u32 + self.reserve as u32,
            false,
        );

        // remain some data, share the same head
        if len > size as u64 + self.reserve as u64 {
            return Some(DataBlock {
                head: self.head,
                data: self
                    .data
                    .allocate(self.data.offset() + size as u64, len - size as u64),
                reserve: 0,
            });
        }
        None
    }

    pub fn finish(&mut self) {
        write_head_end(self.head.as_mut_ptr(), true);
    }
}

impl Cache {
    pub fn new(capacity: usize, name: String) -> Cache {
        let (_, addr) = unsafe {
            let shmpath = name.as_ptr() as *const i8;
            let fd = shm_open(shmpath, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
            let _res = ftruncate(fd, capacity as off_t);
            let addr = mmap(ptr::null_mut(), capacity, PROT_WRITE, MAP_SHARED, fd, 0);
            (fd, addr as *mut u8)
        };
        let head_segment = HeadSegment::new(addr, HEAD_NUM, HEAD_SIZE);
        let data_segment = unsafe {
            DataSegment::new(
                addr.offset(head_segment.size() as isize),
                head_segment.size(),
                capacity as u64 - head_segment.size(),
            )
        };

        Cache {
            name,
            head_size: HEAD_SIZE,
            capacity,
            head_segment,
            data_segment,
            start_addr: addr,
        }
    }

    fn free(&mut self) {
        if let Some(free_heads) = self.head_segment.free() {
            for head in &free_heads {
                loop {
                    let ptr = head.as_mut_ptr();
                    self.data_segment.free(head_off(ptr), head_len(ptr) as u64);
                    if head_end(ptr) {
                        break;
                    }
                }
            }
        }
    }

    pub fn free_block(&mut self, block: DataBlock) {
        // the head is lazy copied
        self.data_segment
            .free(block.data.offset(), block.data.len());
    }

    pub fn allocate_data(&mut self, ref_cnt: usize) -> Buffer {
        // This function return a data
        // Todo(xj): better free method

        let mut data = self.data_segment.allocate();
        if let Some(data) = data {
            return data;
        }
        loop {
            self.free();
            data = self.data_segment.allocate();
            if let Some(data) = data {
                return data;
            }
            thread::sleep(time::Duration::from_secs_f32(0.01));
        }
    }

    pub fn next_block(&mut self, block: Option<DataBlock>, ref_cnt: usize) -> DataBlock {
        if let Some(mut block) = block {
            let mut data = self.allocate_data(ref_cnt);
            // copy the last 16 bytes of last block to the new block
            // the last 16 bytes is the new head
            let head = block.data.allocate(
                block.data.offset() + block.data.len() - self.head_size as u64,
                self.head_size as u64,
            );

            return DataBlock {
                head,
                data,
                reserve: self.head_size as isize,
            };
        }
        let head = self.head_segment.allocate(ref_cnt);
        let data = self.allocate_data(ref_cnt);
        DataBlock {
            head,
            data,
            reserve: 0,
        }
    }

    pub fn start_addr(&self) -> *mut u8 {
        self.start_addr
    }

    pub fn close(&mut self) {
        unsafe {
            let shmpath = self.name.as_ptr() as *const i8;
            shm_unlink(shmpath);
        }
    }
}

#[cfg(test)]
mod test {
    use std::{cmp::min, slice::from_raw_parts};

    use crate::cache::{head_end, head_len, head_off};

    use super::{Cache, HEAD_SIZE};

    struct S {
        a: i32,
        b: i32,
    }
    #[test]
    fn single_thread_test() {
        let len = 256;
        let name = "DLCache".to_string();
        let mut cache = Cache::new(len, name);

        let size_list = &[4, 6, 10];
        let mut off_list = vec![];
        for size in size_list {
            let off = write(&mut cache, *size);
            off_list.push(off);
        }
        for (size, off) in size_list.iter().zip(off_list.iter()) {
            let data = read(*off, cache.start_addr());
            assert_eq!(data.len(), *size);
        }
    }

    fn write(cache: &mut Cache, mut len: usize) -> u64 {
        let data = 7u8;
        let mut block = cache.next_block(None, 0);
        let offset = block.head.offset();
        let size = block.size();
        let write_size = min(len, size as usize);
        let block_slice = block.as_mut_slice();
        for i in 0..write_size {
            block_slice[i] = data;
        }
        let mut remain_block = block.occupy(write_size as usize);
        len -= write_size;
        loop {
            // write flow:
            // allocate block -> write -> occupy(size)
            // if size < block, then some space remain
            // if size = block, then return None
            // if size == 0, then finish writing and free current block
            let mut last_block = block;
            if let Some(_b) = remain_block {
                block = _b;
            } else {
                block = cache.next_block(Some(last_block), 0);
            }
            let size = block.size();
            let write_size = min(len, size as usize);
            let block_slice = block.as_mut_slice();
            for i in 0..write_size {
                block_slice[i] = data;
            }
            remain_block = block.occupy(write_size as usize);
            len -= write_size;
            if len == 0 {
                cache.free_block(block);
                last_block.finish();
                break;
            }
        }

        offset
    }

    fn read(offset: u64, start_addr: *mut u8) -> Vec<u8> {
        let mut addr = unsafe { start_addr.offset(offset as isize) };
        let mut end = head_end(addr);
        let mut len = head_len(addr);
        let mut off = head_off(addr);
        let mut res = Vec::new();
        loop {
            let data = unsafe { from_raw_parts(start_addr.offset(off as isize), len as usize) };
            // res.copy_from_slice(.as_slice())
            for d in data {
                res.push(*d)
            }
            if end {
                break;
            }
            addr = unsafe { start_addr.offset((len - HEAD_SIZE) as isize) };
            end = head_end(addr);
            len = head_len(addr);
            off = head_off(addr);
            for _ in 0..HEAD_SIZE {
                res.pop();
            }
        }
        res
    }
}
