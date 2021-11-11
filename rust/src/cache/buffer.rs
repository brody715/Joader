use std::slice::from_raw_parts_mut;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Buffer {
    // the offset is global for total cache
    offset: u64,
    ptr: *mut u8,
    len: u64,
}

impl Buffer {
    pub fn new(ptr: *mut u8, offset: u64, len: u64) -> Buffer {
        Buffer { offset, ptr, len }
    }

    pub fn as_slice(&self) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self.ptr, self.len as usize) }
    }

    pub fn from_slice(slice: &mut [u8], offset: u64) -> Buffer {
        unsafe { Buffer::new(slice.as_mut_ptr(), offset, slice.len() as u64) }
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn copy_from_slice(&mut self, data: &mut [u8]) {
        unsafe {
            if data.len() > self.len as usize {
                panic!()
            }
            self.ptr.copy_from(data.as_ptr(), data.len())
        }
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.ptr
    }

    pub fn copy_from_buffer(&self, buf: &Buffer) {
        unsafe {
            if buf.len() > self.len {
                panic!()
            }
            self.ptr.copy_from(buf.as_ptr(), buf.len() as usize)
        }
    }

    pub fn get_idx(&self, idx: isize) -> u8 {
        unsafe { *self.ptr.offset(idx) }
    }

    pub fn set_idx(&mut self, idx: isize, byte: u8) {
        unsafe { *self.ptr.offset(idx) = byte };
    }

    pub fn allocate(&mut self, off: u64, len: u64) -> Buffer {
        unsafe { Buffer::new(self.ptr.offset((off - self.offset) as isize), off, len) }
    }
}
