use crate::cache::head::Head;
use crate::cache::{Buffer, DataSegment, HeadSegment};
use core::time;
use libc::{c_void, ftruncate, mmap, shm_open};
use libc::{off_t, shm_unlink};
use libc::{MAP_SHARED, O_CREAT, O_RDWR, PROT_WRITE, S_IRUSR, S_IWUSR};
use std::slice::from_raw_parts_mut;
use std::{ptr, thread};

pub struct Cache {
    name: String,
    capacity: usize,
    head_segment: HeadSegment,
    data_segment: DataSegment,
    start_ptr: *mut u8,
}

const HEAD_NUM: u64 = 8;

#[derive(Debug, Clone, Copy)]
pub struct DataBlock {
    head: Head,
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

        // lazy copy head to the front of the block
        if self.reserve != 0 {
            self.data.copy_from_slice(self.head.as_mut_slice(), 0);
        }

        // write head the meta information
        let len = self.data.len();
        self.head
            .set(false, size as u32 + self.reserve as u32, self.data.offset());

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
        self.head.set_end(true);
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
        let head_segment = HeadSegment::new(addr, HEAD_NUM);
        let data_segment = unsafe {
            DataSegment::new(
                addr.offset(head_segment.size() as isize),
                head_segment.size(),
                capacity as u64 - head_segment.size(),
            )
        };

        Cache {
            name,
            capacity,
            head_segment,
            data_segment,
            start_ptr: addr,
        }
    }

    fn free(&mut self) {
        if let Some(mut unvalid_heads) = self.head_segment.free() {
            for head in unvalid_heads.iter_mut() {
                loop {
                    let (end, len, off) = head.get();
                    self.data_segment.free(off, len as u64);
                    if end {
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

    pub fn next_block(&mut self, block: Option<DataBlock>, ref_cnt: usize) -> (DataBlock, usize) {
        // next block
        if let Some(mut block) = block {
            let data = self.allocate_data(ref_cnt);
            // copy the last 16 bytes of last block to the new block
            // the last 16 bytes is the new head
            let head = block
                .data
                .allocate(
                    block.data.offset() + block.data.len() - Head::size() as u64,
                    Head::size(),
                )
                .into();
            return (
                DataBlock {
                    head,
                    data,
                    reserve: Head::size() as isize,
                },
                0,
            );
        }
        // first block
        let (head, idx) = self.head_segment.allocate(ref_cnt);
        let data = self.allocate_data(ref_cnt);
        (
            DataBlock {
                head,
                data,
                reserve: 0,
            },
            idx,
        )
    }

    pub fn start_ptr(&self) -> *mut u8 {
        self.start_ptr
    }

    pub fn close(&mut self) {
        unsafe {
            let shmpath = self.name.as_ptr() as *const i8;
            shm_unlink(shmpath);
        }
    }

    fn print(&mut self) {
        // print head
        for i in 0..self.head_segment.size() / super::head::HEAD_SIZE {
            unsafe {
                let mut head: super::head::Head =
                    (self.start_ptr.offset((i * super::head::HEAD_SIZE) as isize)).into();
                print!("{:?}\n", head.get());
            }
        }
        print!("{:?}", self.data_segment.data().as_slice());
        // print data
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{cmp::min, slice::from_raw_parts};
    #[test]
    fn single_thread_test() {
        let len = 256;
        let name = "DLCache".to_string();
        let mut cache = Cache::new(len, name);

        let size_list = &[20, 27, 60, 19];
        let mut idx_list = vec![];
        for size in size_list {
            let idx = write(&mut cache, *size, size % 2, 7);
            idx_list.push(idx);
        }
        cache.print();
        for (size, off) in size_list.iter().zip(idx_list.iter()) {
            let data = read(*off, cache.start_ptr(), 7);
            assert_eq!(data.len(), *size);
        }

        // some data should be free
        let size_list = &[40];
        let mut idx_list = vec![];
        for size in size_list {
            let idx = write(&mut cache, *size, size % 3, 3);
            idx_list.push(idx);
        }
        cache.print();
        // for (size, off) in size_list.iter().zip(idx_list.iter()) {
        //     let data = read(*off, cache.start_ptr(), 3);
        //     assert_eq!(data.len(), *size);
        // }
        cache.close()
    }

    fn write(cache: &mut Cache, mut len: usize, ref_cnt: usize, value: u8) -> usize {
        let (mut block, idx) = cache.next_block(None, ref_cnt);
        let mut block_slice = block.as_mut_slice();
        let mut write_size = min(len, block_slice.len());
        (0..write_size).fold((), |_, i| block_slice[i] = value);
        let mut remain_block = block.occupy(write_size as usize);
        len -= write_size;

        loop {
            let mut last_block = block;
            // write flow:
            // allocate block -> write -> occupy(size)
            // if size < block, then some space remain
            // if size = block, then return None
            // if size == 0, then finish writing and free current block
            if let Some(_b) = remain_block {
                block = _b;
            } else {
                block = cache.next_block(Some(last_block), 0).0;
            }

            block_slice = block.as_mut_slice();
            write_size = min(len, block_slice.len());

            (0..write_size).fold((), |_, i| block_slice[i] = value);
            remain_block = block.occupy(write_size as usize);
            len -= write_size;
            if write_size == 0 {
                cache.free_block(block);
                last_block.finish();
                break;
            }
        }
        idx
    }

    fn read(idx: usize, start_ptr: *mut u8, value: u8) -> Vec<u8> {
        let mut addr = unsafe { start_ptr.offset((idx as isize) * (Head::size() as isize)) };
        let mut head = Head::from(addr);
        let (mut end, mut len, mut off) = head.get();

        let mut res = Vec::new();
        loop {
            let data = unsafe { from_raw_parts(start_ptr.offset(off as isize), len as usize) };
            for d in data {
                res.push(*d);
                assert_eq!(*d, value);
            }
            if end {
                break;
            }
            addr = unsafe { start_ptr.offset(len as isize - Head::size() as isize) };
            let value = Head::from(addr).get();
            end = value.0;
            len = value.1;
            off = value.2;
            for _ in 0..Head::size() {
                res.pop();
            }
        }
        head.set_unvalid();
        res
    }
}
