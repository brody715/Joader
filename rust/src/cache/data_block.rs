use std::slice::{from_raw_parts, from_raw_parts_mut};

use libc::c_void;

use crate::cache::head::{Head, HEAD_SIZE};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Data {
    // the ptr is the point of data start, that is the global ptr + off
    ptr: *mut u8,
    // the size of data is less than 4GB
    len: u64,
    off: u64,
}

impl Data {
    pub fn new(ptr: *mut u8, off: u64, len: u64) -> Data {
        Data { ptr, len, off }
    }

    pub fn allocate(&mut self, off: u64, len: u64) -> Data {
        Data {
            ptr: unsafe { self.ptr.offset(off as isize - self.off as isize) },
            off,
            len,
        }
    }

    pub fn tail_head(&mut self) -> Head {
        unsafe { self.ptr.offset((self.len - HEAD_SIZE) as isize) }.into()
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self.ptr, self.len as usize) }
    }

    pub fn as_slice(&mut self) -> &[u8] {
        unsafe { from_raw_parts(self.ptr, self.len as usize) }
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn off(&self) -> u64 {
        self.off
    }

    pub fn copy_head(&mut self, head: Head) {
        assert!(self.len > HEAD_SIZE);
        unsafe {
            self.ptr.copy_from(head.as_ptr(), HEAD_SIZE as usize);
        }
    }

    pub fn remain(&mut self, occupy_size: u64) -> Option<Data> {
        assert!(occupy_size <= self.len);
        if occupy_size == self.len {
            return None;
        }
        Some(Data {
            ptr: unsafe { self.ptr.offset(occupy_size as isize) },
            len: self.len - occupy_size,
            off: self.off + occupy_size,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DataBlock {
    pub head: Head,
    pub data: Data,
    // if it's true, we should transfer the data in head to data
    pub transfer: bool,
}

impl DataBlock {
    pub fn size(&self) -> u64 {
        if self.transfer {
            self.data.len() - HEAD_SIZE
        } else {
            self.data.len()
        }
    }

    pub fn ptr(&mut self) -> *mut c_void {
        let mut offset = 0;
        if self.transfer {
            offset = HEAD_SIZE as isize;
        }

        unsafe { self.data.as_mut_ptr().offset(offset).cast::<c_void>() }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        let mut offset = 0;
        if self.transfer {
            offset = HEAD_SIZE as isize;
        }
        unsafe { from_raw_parts_mut(self.data.as_mut_ptr().offset(offset), self.size() as usize) }
    }

    pub fn occupy(&mut self, size: usize) -> Option<DataBlock> {
        if size == 0 {
            return Some(*self);
        }

        // lazy copy head to the front of the block
        if self.transfer {
            self.data.copy_head(self.head);
        }

        // write head the meta information
        let mut size = size as u64;
        if self.transfer {
            size += HEAD_SIZE;
        }
        self.head.set(false, size as u32, self.data.off());

        // remain some data, share the same head
        if let Some(data) = self.data.remain(size) {
            return Some(DataBlock {
                head: self.head,
                data,
                transfer: false,
            });
        }
        None
    }

    pub fn finish(&mut self) {
        self.head.set_end(true);
    }

    pub fn data(&mut self) -> Data {
        self.data
    }
}
