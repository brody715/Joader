use core::time;
use std::{ptr, thread};

use super::{head_end, head_len, head_off, Buffer, DataSegment, HeadSegment};
use libc::{ftruncate, mmap, shm_open};
use libc::{off_t, shm_unlink};
use libc::{MAP_SHARED, O_CREAT, O_RDWR, PROT_WRITE, S_IRUSR, S_IWUSR};

pub struct Cache {
    name: String,
    capacity: usize,
    head_size: u32,
    head_segment: HeadSegment,
    data_segment: DataSegment,
}

pub struct DataBlock {
    head: Buffer,
    data: Buffer,
}

impl Cache {
    pub fn new(capacity: usize, name: String) -> Cache {
        const HEAD_SIZE: u32 = 16;
        const HEAD_NUM: u64 = 4096;
        let (_, addr) = unsafe {
            let shmpath = name.as_ptr() as *const i8;
            let fd = shm_open(shmpath, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
            let _res = ftruncate(fd, capacity as off_t);
            let addr = mmap(ptr::null_mut(), capacity, PROT_WRITE, MAP_SHARED, fd, 0);
            (fd, addr as *mut u8)
        };
        let head_segment = HeadSegment::new(addr, HEAD_NUM, HEAD_SIZE);
        let data_segment = DataSegment::new(
            addr,
            head_segment.size(),
            capacity as u64 - head_segment.size(),
        );

        Cache {
            name,
            head_size: HEAD_SIZE,
            capacity,
            head_segment,
            data_segment,
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
            data.copy_from_buffer(&head, 0);

            return DataBlock {
                head,
                data: data.allocate(
                    data.offset() + self.head_size as u64,
                    data.len() - self.head_size as u64,
                ),
            };
        }
        let head = self.head_segment.allocate(ref_cnt);
        let data = self.allocate_data(ref_cnt);
        DataBlock { head, data }
    }

    pub fn finish(&mut self, block: &DataBlock, len: u32) {
        let head_ptr = block.head.as_mut_ptr();
        let off = block.data.offset();
        HeadSegment::write_head(head_ptr, off, len, true);
        self.data_segment
            .free(off + len as u64, block.data.len() - len as u64);
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
    struct S {
        a: i32,
        b: i32,
    }
    #[test]
    fn test() {
        let mut s = S { a: 1, b: 2 };
        let mut _x = &mut s.a;
        let mut _y = &mut s.b;
    }
}
