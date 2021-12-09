use crate::cache::cached_data::CachedData;
use crate::cache::data_segment::DataSegment;
use crate::cache::head_segment::HeadSegment;
use core::time;
use libc::{ftruncate, mmap, shm_open};
use libc::{off_t, shm_unlink};
use libc::{MAP_SHARED, O_CREAT, O_RDWR, PROT_WRITE, S_IRUSR, S_IWUSR};
use std::slice::from_raw_parts_mut;
use std::time::Duration;
use std::{ptr, thread, usize};

use super::data_block::{Data, DataBlock};
use super::head::{Head, HEAD_SIZE};

#[derive(Debug)]
pub struct Cache {
    shmpath: String,
    head_segment: HeadSegment,
    data_segment: DataSegment,
    cached_data: CachedData,
    start_ptr: *mut u8,
    sleep_iterver: Duration,
}
unsafe impl Send for Cache {}

impl Cache {
    pub fn new(capacity: usize, shmpath: &str, head_num: u64) -> Cache {
        let (_, addr) = unsafe {
            let shmpath = shmpath.as_ptr() as *const i8;
            let fd = shm_open(shmpath, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
            let _res = ftruncate(fd, capacity as off_t);
            let addr = mmap(ptr::null_mut(), capacity, PROT_WRITE, MAP_SHARED, fd, 0);
            // Todo(xj): It's just avoid empty file, we should add a magic code in the front of the file
            *(addr as *mut u8).offset(3) = 5u8;
            (fd, addr as *mut u8)
        };
        let head_segment = HeadSegment::new(addr, head_num);
        let data_segment = unsafe {
            DataSegment::new(
                addr.offset(head_segment.size() as isize),
                head_segment.size(),
                capacity as u64 - head_segment.size(),
            )
        };

        Cache {
            shmpath: shmpath.to_string(),
            head_segment,
            data_segment,
            start_ptr: addr,
            cached_data: CachedData::new(),
            sleep_iterver: time::Duration::from_secs_f32(0.001),
        }
    }

    fn free(&mut self) {
        if let Some(mut unvalid_heads) = self.head_segment.free() {
            for (head, idx) in unvalid_heads.iter_mut() {
                self.cached_data.remove(*idx);
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

    pub fn allocate(&mut self, len: usize, ref_cnt: usize, data_name: &str) -> (&mut [u8], usize) {
        let (head, idx) = self.allocate_head(ref_cnt);
        self.cached_data.add(idx, data_name);
        let data = self.allocate_data(len as u64);
        let mut block = DataBlock {
            head,
            data,
            transfer: false,
        };
        if let Some(block) = block.occupy(len) {
            self.free_block(block);
        }
        block.finish();
        let ptr = block.data().as_mut_ptr();
        return (unsafe { from_raw_parts_mut(ptr, len) }, idx);
    }

    fn allocate_data(&mut self, request_len: u64) -> Data {
        // This function return a data or loop
        // Todo(xj): better free method
        let mut data = self.data_segment.allocate();
        if let Some(data) = data {
            if data.len() >= request_len {
                return data;
            }
            self.data_segment.free(data.off(), data.len());
        }
        loop {
            self.free();
            data = self.data_segment.allocate();
            if let Some(data) = data {
                if data.len() >= request_len {
                    return data;
                }
                self.data_segment.free(data.off(), data.len());
            }
            thread::sleep(self.sleep_iterver);
            log::debug!("Loop in allocate data");
        }
    }

    fn allocate_head(&mut self, ref_cnt: usize) -> (Head, usize) {
        let mut ret = self.head_segment.allocate(ref_cnt);
        if let Some((head, idx)) = ret {
            return (head, idx);
        }
        loop {
            self.free();
            ret = self.head_segment.allocate(ref_cnt);
            if let Some((head, idx)) = ret {
                return (head, idx);
            }
            thread::sleep(self.sleep_iterver);
            log::debug!("Loop in allocate head");
        }
    }

    pub fn contains_data(&self, data: &str) -> Option<usize> {
        self.cached_data.contains(data)
    }

    pub fn next_block(
        &mut self,
        block: Option<DataBlock>,
        ref_cnt: usize,
        data_name: &str,
    ) -> (DataBlock, usize) {
        let data = self.allocate_data(0);
        // next block
        if let Some(mut block) = block {
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
        let (head, idx) = self.allocate_head(ref_cnt);
        self.cached_data.add(idx, data_name);
        return (
            DataBlock {
                head,
                data,
                transfer: false,
            },
            idx,
        );
    }

    pub fn start_ptr(&self) -> *mut u8 {
        self.start_ptr.clone()
    }

    pub fn close(shmpath: String) {
        unsafe {
            let shmpath = shmpath.as_ptr() as *const i8;
            shm_unlink(shmpath);
        }
    }

    pub fn get_shm_path(&self) -> &str {
        &self.shmpath
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cache::head::{Head, HEAD_SIZE};
    use crossbeam::channel::{unbounded, Receiver, Sender};
    use std::{cmp::min, slice::from_raw_parts, sync::atomic::AtomicPtr, time::SystemTime};
    #[test]
    fn single_thread_test() {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        let len = 256;
        let name = "DLCache".to_string();
        let head_num = 8;
        let mut cache = Cache::new(len, &name, head_num);

        let size_list = &[(20, 0), (27, 1), (60, 2), (20, 3)];
        let mut idx_list = vec![];
        for (idx, (size, ref_cnt)) in size_list.iter().enumerate() {
            let idx = write(&mut cache, *size, *ref_cnt, 7, &idx.to_string());
            idx_list.push(idx);
        }
        for ((size, _), off) in size_list.iter().zip(idx_list.iter()) {
            let data = read(*off, cache.start_ptr(), 7);
            assert_eq!(data.len(), *size);
        }

        // some data should be free
        let size_list = &[40, 38];
        let mut idx_list = vec![];
        for (idx, size) in size_list.iter().enumerate() {
            let idx = write(&mut cache, *size, size % 2, 3, &idx.to_string());
            idx_list.push(idx);
        }
        for (size, off) in size_list.iter().zip(idx_list.iter()) {
            let data = read(*off, cache.start_ptr(), 3);
            assert_eq!(data.len(), *size);
        }

        // some data should be free
        let size_list = &[127];
        let mut idx_list = vec![];
        for (idx, size) in size_list.iter().enumerate() {
            let idx = write(&mut cache, *size, size % 3, 5, &idx.to_string());
            idx_list.push(idx);
        }
        for (size, off) in size_list.iter().zip(idx_list.iter()) {
            let data = read(*off, cache.start_ptr(), 5);
            assert_eq!(data.len(), *size);
        }
        Cache::close(name);
    }

    #[test]
    fn two_thread_test() {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        const TURN: usize = 10000;
        let head_num: usize = 128;
        let len = (HEAD_SIZE as usize) * ((head_num * 2 + TURN) * HEAD_SIZE as usize);
        let name = "DLCache".to_string();
        let (wc, rc) = unbounded::<usize>();
        let (addr_wc, addr_rc) = unbounded();
        let writer = thread::spawn(move || {
            let cache = Cache::new(len, &name, head_num as u64);
            log::debug!("writer start {:?}", cache.start_ptr());
            addr_wc.send(AtomicPtr::new(cache.start_ptr())).unwrap();
            writer_func(cache, TURN, wc);
            log::debug!("write finish.......");
            thread::sleep(time::Duration::from_secs(5));
            Cache::close(name);
        });
        let reader = thread::spawn(move || {
            let mut start_ptr = addr_rc.recv().unwrap();
            log::debug!("reader start {:?}", *start_ptr.get_mut());
            reader_func(*start_ptr.get_mut(), TURN, rc);
            println!("read finish.......");
        });
        reader.join().unwrap();
        writer.join().unwrap();
    }

    fn writer_func(mut cache: Cache, turn: usize, wc: Sender<usize>) {
        let mut start = SystemTime::now();
        for i in 1..turn {
            let len = i * HEAD_SIZE as usize;
            let idx = {
                let (block_slice, idx) = cache.allocate(len, i % 3, i.to_string().as_str());
                block_slice.copy_from_slice(vec![7u8; len].as_slice());
                idx
            };
            wc.send(idx).unwrap();
            if i % 1000 == 0 {
                println!(
                    "write..{:} avg time: {:}",
                    i,
                    SystemTime::now().duration_since(start).unwrap().as_secs() as f64 / 1000 as f64
                );
                start = SystemTime::now();
            }
        }
        drop(wc);
    }

    fn reader_func(start_ptr: *mut u8, turn: usize, rc: Receiver<usize>) {
        let mut start = SystemTime::now();
        for i in 1..turn {
            if i % 1000 == 0 {
                println!(
                    "read..{:} avg time: {:}",
                    i,
                    SystemTime::now().duration_since(start).unwrap().as_secs() as f64 / 1000 as f64
                );
                start = SystemTime::now();
            }
            let idx = rc.recv().unwrap();
            let addr = unsafe { start_ptr.offset((idx as isize) * (Head::size() as isize)) };
            let mut head = Head::from(addr);
            let (end, len, off) = head.get();
            assert!(end == true);
            let data = unsafe { from_raw_parts(start_ptr.offset(off as isize), len as usize) };
            assert_eq!(data.len(), i * (HEAD_SIZE as usize));
            data.iter().fold((), |_, x| assert_eq!(*x, 7));
            head.readed();
        }
        drop(rc);
    }

    fn write(
        cache: &mut Cache,
        mut len: usize,
        ref_cnt: usize,
        value: u8,
        data_name: &str,
    ) -> usize {
        let (mut block, idx) = cache.next_block(None, ref_cnt, data_name);
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
                block = cache.next_block(Some(last_block), 0, data_name).0;
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
            let data;
            if end {
                data = unsafe { from_raw_parts(start_ptr.offset(off as isize), len as usize) };
                log::debug!("read [{:?}, {:?})", off, off + len as u64);
                data.iter().fold((), |_, x| {
                    assert!(*x == value);
                    res.push(*x)
                });
                break;
            } else {
                data = unsafe {
                    log::debug!("read [{:?}, {:?})", off, off + len as u64 - HEAD_SIZE);
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
