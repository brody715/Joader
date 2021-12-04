use std::{convert::TryInto, slice::from_raw_parts, slice::from_raw_parts_mut};

// head: |end|read|--len--|----off----|
//       |1|1|-|-|--4--|----8----|
pub const HEAD_SIZE: u64 = 16;

#[derive(Debug, Clone, Copy)]
pub struct Head {
    ptr: *mut u8,
    free: bool,
}

impl From<*mut u8> for Head {
    fn from(ptr: *mut u8) -> Self {
        Self { ptr, free: true }
    }
}

impl Head {
    pub fn set(&mut self, end: bool, len: u32, off: u64) {
        log::debug!("Write head end: {:?}, off: {:} len:{:})", end, off, len);
        self.set_end(end);
        self.set_len(len);
        self.set_off(off);
    }

    pub fn set_end(&mut self, end: bool) {
        unsafe { self.ptr.copy_from((end as u8).to_be_bytes().as_ptr(), 1) };
    }

    fn set_len(&mut self, len: u32) {
        unsafe { self.ptr.offset(4).copy_from(len.to_be_bytes().as_ptr(), 4) };
    }

    fn set_off(&mut self, off: u64) {
        unsafe { self.ptr.offset(8).copy_from(off.to_be_bytes().as_ptr(), 8) };
    }

    pub fn get(&self) -> (bool, u32, u64) {
        (self.get_end(), self.get_len(), self.get_off())
    }

    pub fn get_len(&self) -> u32 {
        let slice = unsafe { from_raw_parts(self.ptr.offset(4), 4) };

        u32::from_be_bytes(slice.try_into().unwrap())
    }

    pub fn get_off(&self) -> u64 {
        let slice = unsafe { from_raw_parts(self.ptr.offset(8), 8) };
        u64::from_be_bytes(slice.try_into().unwrap())
    }

    pub fn get_end(&self) -> bool {
        let slice = unsafe { from_raw_parts(self.ptr, 1) };
        u8::from_be_bytes(slice.try_into().unwrap()) == 1
    }

    pub fn is_readed(&self) -> bool {
        let slice = unsafe { from_raw_parts(self.ptr.offset(1), 1) };
        u8::from_be_bytes(slice.try_into().unwrap()) == 0
    }

    pub fn readed(&mut self) {
        unsafe {
            self.ptr
                .offset(1)
                .copy_from((0 as u8).to_be_bytes().as_ptr(), 1)
        };
    }

    pub fn is_free(&self) -> bool {
        self.free
    }

    pub fn set_free(&mut self) {
        self.free = true;
    }

    pub fn allocated(&mut self) {
        self.free = false;
        unsafe {
            self.ptr
                .offset(1)
                .copy_from((1 as u8).to_be_bytes().as_ptr(), 1)
        };
    }

    pub fn size() -> u64 {
        HEAD_SIZE
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self.ptr, HEAD_SIZE as usize) }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }
}
