use std::{convert::TryInto, slice::from_raw_parts, slice::from_raw_parts_mut};

use crate::cache::Buffer;

// head: |end|valid|--len--|----off----|
//       |1|1|-|-|--4--|----8----|
pub const HEAD_SIZE: u64 = 16;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum HeadState {
    Free,
    Valid,
    Unvalid,
}

#[derive(Debug, Clone, Copy)]
pub struct Head {
    ptr: *mut u8,
    // state for each head, 0 : free, 1: valid, 2: unvalid
    // 0 -> 1 -> 2 -> 0
    state: HeadState,
}

impl From<Buffer> for Head {
    fn from(buf: Buffer) -> Self {
        assert_eq!(buf.len(), HEAD_SIZE);
        Self {
            ptr: buf.as_mut_ptr(),
            state: HeadState::Free,
        }
    }
}

impl From<*mut u8> for Head {
    fn from(ptr: *mut u8) -> Self {
        Self {
            ptr,
            state: HeadState::Free,
        }
    }
}

impl Head {
    pub fn set(&mut self, end: bool, len: u32, off: u64) {
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

    pub fn get(&mut self) -> (bool, u32, u64) {
        (self.get_end(), self.get_len(), self.get_off())
    }

    fn get_len(&mut self) -> u32 {
        let slice = unsafe { from_raw_parts(self.ptr.offset(4), 4) };

        u32::from_be_bytes(slice.try_into().unwrap())
    }

    fn get_off(&mut self) -> u64 {
        let slice = unsafe { from_raw_parts(self.ptr.offset(8), 8) };
        u64::from_be_bytes(slice.try_into().unwrap())
    }

    fn get_end(&mut self) -> bool {
        let slice = unsafe { from_raw_parts(self.ptr, 1) };
        u8::from_be_bytes(slice.try_into().unwrap()) == 1
    }

    pub fn is_valid(&self) -> bool {
        let slice = unsafe { from_raw_parts(self.ptr.offset(1), 1) };
        self.state == HeadState::Valid || u8::from_be_bytes(slice.try_into().unwrap()) == 0
    }

    pub fn is_free(&self) -> bool {
        self.state == HeadState::Free
    }

    pub fn is_unvalid(&self) -> bool {
        let slice = unsafe { from_raw_parts(self.ptr.offset(1), 1) };
        self.state == HeadState::Unvalid || u8::from_be_bytes(slice.try_into().unwrap()) == 1
    }

    pub fn set_valid(&mut self) {
        self.state = HeadState::Valid;
    }

    pub fn set_free(&mut self) {
        self.state = HeadState::Free;
    }

    pub fn set_unvalid(&mut self) {
        self.state = HeadState::Unvalid;
        // Todo(xj): better unvalid method
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
}
