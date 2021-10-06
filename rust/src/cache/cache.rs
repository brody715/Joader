use std::ptr;

use super::HeadMap;
use crate::cache::freelist::FreeList;
use libc::{c_char, c_void, off_t, shm_unlink, size_t};
use libc::{close, ftruncate, memcpy, mmap, shm_open, strncpy};
use libc::{MAP_SHARED, O_CREAT, O_RDWR, PROT_WRITE, S_IRUSR, S_IWUSR};




pub struct Cache {
    name: String,
    addr: *mut u8,
    capacity: usize,
    head_map: HeadMap,
    free_list: FreeList,
    head_cursor: *mut u8,
    block_cursor: *mut u8,
}

impl Cache {
    pub fn new(capacity: usize, name: String) -> Cache {
        let (fd, addr) = unsafe {
            let shmpath = name.as_ptr() as *const i8;
            let fd = shm_open(shmpath, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
            let _res = ftruncate(fd, capacity as off_t);
            let addr = mmap(ptr::null_mut(), capacity, PROT_WRITE, MAP_SHARED, fd, 0);
            (fd, addr as *mut u8)
        };
        let head_map = HeadMap::new();
        let free_list = FreeList::new();
        Cache {
            name,
            addr,
            capacity,
            head_map,
            free_list,
            head_cursor: ptr::null_mut(),
            block_cursor: ptr::null_mut(),
        }
    }
    pub fn allocate(&mut self) -> &mut [u8] {
        todo!()
    }

    pub fn finish(&mut self, data_size: usize, end: bool) -> &mut [u8] {
        todo!()
    }

    pub fn close(&mut self) {
        unsafe {
            let shmpath = self.name.as_ptr() as *const i8;
            shm_unlink(shmpath);
        }
    }
}
