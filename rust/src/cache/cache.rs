use std::os::unix::net::Incoming;
use std::ptr;
use std::slice::from_raw_parts_mut;

use super::{Buffer, DataSegment, HeadSegment};
use libc::{ off_t, shm_unlink};
use libc::{ftruncate, mmap, shm_open};
use libc::{MAP_SHARED, O_CREAT, O_RDWR, PROT_WRITE, S_IRUSR, S_IWUSR};

pub struct Cache {
    name: String,
    capacity: usize,
    head_segment: HeadSegment,
    data_segment: DataSegment,
    head_cursor: Option<Buffer>,
}

impl Cache {
    pub fn new(capacity: usize, name: String) -> Cache {
        todo!()
        // let (_, addr) = unsafe {
        //     let shmpath = name.as_ptr() as *const i8;
        //     let fd = shm_open(shmpath, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
        //     let _res = ftruncate(fd, capacity as off_t);
        //     let addr = mmap(ptr::null_mut(), capacity, PROT_WRITE, MAP_SHARED, fd, 0);
        //     (fd, addr as *mut u8)
        // };
        // let (head_segment, size) = HeadSegment::new(addr, 4096, 128);
        // let data_segment = DataSegment::new(addr, size, capacity-size);

        // Cache {
        //     name,
        //     capacity,
        //     head_segment,
        //     data_segment,
        //     head_cursor: None,
        // }
    }

    pub fn next_block(&mut self, block: Buffer, end: bool) -> Option<Buffer> {
        todo!()
        // if let None = self.head_cursor {
        //     self.head_cursor = Some(self.head_segment.allocate());
        //     return self.data_segment.allocate();
        // }
        // let head = self.head_cursor.clone().unwrap();
        // HeadSegment::write_head(head, block, end);
        // self.head_cursor = None;
        // let mut next_block = None;
        // if !end {
        //     next_block = self.data_segment.allocate();
        //     let ret = HeadSegment::replace_head(block, next_block.unwrap());
        //     self.head_cursor = Some(ret.0);
        //     next_block = Some(ret.1);
        // }
        // next_block
    }

    pub fn close(&mut self) {
        todo!()
        // unsafe {
        //     let shmpath = self.name.as_ptr() as *const i8;
        //     shm_unlink(shmpath);
        // }
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
        let mut s = S{a:1, b:2};
        let mut _x = &mut s.a;
        let mut _y = &mut s.b;
    }
}
