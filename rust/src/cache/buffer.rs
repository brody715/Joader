use std::slice::from_raw_parts_mut;

#[derive(Debug, Clone, Copy)]
pub struct Buffer {
    offset: usize,
    ptr: *mut u8,
    len: usize,
}

impl Buffer {
    pub fn new(ptr: *mut u8, offset: usize, len: usize) -> Buffer {
        Buffer {offset, ptr, len}
    }

    pub fn as_slice(&self) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self.ptr, self.len) }
    }

    pub fn from_slice(slice: &mut [u8]) -> Buffer {
        unsafe {Buffer::new(slice.as_mut_ptr(), slice.len())}
    }

    pub fn copy_from_slice(&mut self, data: &mut [u8]) {
        unsafe {
            if data.len() > self.len {
                panic!()
            }
            self.ptr.copy_from(data.as_ptr(), data.len())
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }

    pub fn copy_from_buffer(&self, buf: &Buffer) {
        unsafe {
            if buf.len() > self.len {
                panic!()
            }
            self.ptr.copy_from(buf.as_ptr(), buf.len())
        }
    }

    pub fn get_idx(&self, idx: isize) -> u8 {
        unsafe { *self.ptr.offset(idx) }
    }

    pub fn set_idx(&mut self, idx: isize, byte: u8) {
        unsafe { *self.ptr.offset(idx) = byte };
    }

    pub fn allocate(&mut self, off: isize, len: usize) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self.ptr.offset(off), len) }
    }
}
