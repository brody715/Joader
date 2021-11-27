use crate::cache::data_segment::DataSegment;
use crate::cache::head_segment::HeadSegment;
use core::time;
use libc::{ftruncate, mmap, shm_open};
use libc::{off_t, shm_unlink};
use libc::{MAP_SHARED, O_CREAT, O_RDWR, PROT_WRITE, S_IRUSR, S_IWUSR};
use std::{ptr, thread};

use super::data_block::{Data, DataBlock};
use super::head::{Head, HEAD_SIZE};

pub struct Cache {
    name: String,
    capacity: usize,
    head_segment: HeadSegment,
    data_segment: DataSegment,
    start_ptr: *mut u8,
}

const HEAD_NUM: u64 = 8;

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
                let (mut end, mut len, mut off) = head.get();
                loop {
                    self.data_segment.free(off, len as u64);
                    if end {
                        break;
                    }
                    let head = Head::from(unsafe {
                        self.start_ptr
                            .offset((off + len as u64 - HEAD_SIZE) as isize)
                    });
                    end = head.get_end();
                    len = head.get_len();
                    off = head.get_off();
                }
            }
        }
    }

    pub fn free_block(&mut self, mut block: DataBlock) {
        // the head is lazy copied
        self.data_segment
            .free(block.data().off(), block.data().len());
    }

    pub fn allocate_data(&mut self, ref_cnt: usize) -> Data {
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
            // the last space is the new head
            return (
                DataBlock {
                    head: block.data().tail_head(),
                    data,
                    transfer: true,
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
                transfer: false,
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
                print!("{:?}{:?}\n", head.is_readed(), head.get());
            }
        }
        print!("{:?}", self.data_segment.data().as_mut_slice());
        // print data
    }
}

#[cfg(test)]
mod test {
    use crate::cache::head::{Head, HEAD_SIZE};

    use super::*;
    use std::{cmp::min, slice::from_raw_parts};
    #[test]
    fn single_thread_test() {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        let len = 256;
        let name = "DLCache".to_string();
        let mut cache = Cache::new(len, name);

        let size_list = &[(20, 0), (27, 1), (60, 2), (20, 3)];
        let mut idx_list = vec![];
        for (size, ref_cnt) in size_list {
            let idx = write(&mut cache, *size, *ref_cnt, 7);
            idx_list.push(idx);
        }
        cache.print();
        for ((size, _), off) in size_list.iter().zip(idx_list.iter()) {
            let data = read(*off, cache.start_ptr(), 7);
            assert_eq!(data.len(), *size);
        }

        // some data should be free
        let size_list = &[40, 38];
        let mut idx_list = vec![];
        for size in size_list {
            let idx = write(&mut cache, *size, size % 2, 3);
            idx_list.push(idx);
        }
        cache.print();
        for (size, off) in size_list.iter().zip(idx_list.iter()) {
            let data = read(*off, cache.start_ptr(), 3);
            assert_eq!(data.len(), *size);
        }

        // some data should be free
        let size_list = &[128];
        let mut idx_list = vec![];
        for size in size_list {
            let idx = write(&mut cache, *size, size % 3, 5);
            idx_list.push(idx);
        }
        cache.print();
        for (size, off) in size_list.iter().zip(idx_list.iter()) {
            let data = read(*off, cache.start_ptr(), 5);
            assert_eq!(data.len(), *size);
        }
        cache.close()
    }

    fn write(cache: &mut Cache, mut len: usize, ref_cnt: usize, value: u8) -> usize {
        let (mut block, idx) = cache.next_block(None, ref_cnt);
        let mut block_slice = block.as_mut_slice();
        let mut write_size = min(len, block_slice.len());
        (0..write_size).fold((), |_, i| block_slice[i] = value);
        let mut remain_block = block.occupy(write_size as usize);
        len -= write_size;
        let ss = cache.data_segment.data().as_slice();
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
            let ss = cache.data_segment.data().as_slice();
            block_slice = block.as_mut_slice();
            write_size = min(len, block_slice.len());

            (0..write_size).fold((), |_, i| block_slice[i] = value);
            remain_block = block.occupy(write_size as usize);
            let ss = cache.data_segment.data().as_slice();
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
            let data;
            if end {
                data = unsafe { from_raw_parts(start_ptr.offset(off as isize), len as usize) };
                data.iter().fold((), |_, x| {
                    assert!(*x == value);
                    res.push(*x)
                });
                break;
            } else {
                data = unsafe {
                    from_raw_parts(
                        start_ptr.offset(off as isize),
                        len as usize - HEAD_SIZE as usize,
                    )
                };
                data.iter().fold((), |_, x| {
                    assert!(*x == value);
                    res.push(*x)
                });
            }
            addr = unsafe { start_ptr.offset(off as isize + len as isize - Head::size() as isize) };
            let head = Head::from(addr);
            end = head.get_end();
            len = head.get_len();
            off = head.get_off();
        }
        head.readed();
        res
    }
}
