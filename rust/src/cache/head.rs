use std::{convert::TryInto, io::BufRead, slice::from_raw_parts};

use futures::SinkExt;

use crate::cache::bit::Bitmap;

use super::Buffer;
// head: |--end--|--len--|--off--|
//       |--4--|--4--|--8--|
pub fn write_head_end(addr: *mut u8, end: bool) {
    unsafe { addr.copy_from((end as u32).to_be_bytes().as_ptr(), 4) };
}

pub fn write_head_len(addr: *mut u8, len: u32) {
    unsafe { addr.offset(4).copy_from(len.to_be_bytes().as_ptr(), 4) };
}

pub fn write_head_off(addr: *mut u8, off: u64) {
    unsafe { addr.offset(8).copy_from(off.to_be_bytes().as_ptr(), 8) };
}

pub fn head_len(addr: *mut u8) -> u32 {
    let slice;
    unsafe {
        slice = from_raw_parts(addr.offset(4), 4);
    }

    u32::from_be_bytes(slice.try_into().unwrap())
}

pub fn head_off(addr: *mut u8) -> u64 {
    let slice;
    unsafe {
        slice = from_raw_parts(addr.offset(8), 8);
    }

    u64::from_be_bytes(slice.try_into().unwrap())
}

pub fn head_end(addr: *mut u8) -> bool {
    let slice;
    unsafe {
        slice = from_raw_parts(addr, 4);
    }

    u32::from_be_bytes(slice.try_into().unwrap()) == 1
}

pub fn write_head(head: *mut u8, off: u64, len: u32, end: bool) {
    write_head_off(head, off);
    write_head_end(head, end);
    write_head_len(head, len);
}
pub struct HeadSegment {
    bitmap: Bitmap,
    head: Buffer,
    // 16
    head_size: u32,

    // Record the ref cnt of each data in the sampling tree
    ref_table: Vec<Vec<u64>>,
}

impl HeadSegment {
    pub fn new(addr: *mut u8, head_num: u64, head_size: u32) -> HeadSegment {
        unsafe {
            HeadSegment {
                bitmap: Bitmap::new(Buffer::new(addr, 0, head_num)),
                head: Buffer::new(
                    addr.offset(head_num as isize),
                    head_num as u64,
                    head_num * (head_size as u64),
                ),
                head_size,
                // there are 64 level
                ref_table: vec![Vec::new(); 64],
            }
        }
    }

    pub fn size(&self) -> u64 {
        self.bitmap.len() + self.head.len()
    }

    pub fn allocate(&mut self, ref_cnt: usize) -> Buffer {
        assert!(ref_cnt < 64);
        let idx = self.bitmap.find_free();
        self.bitmap.set(idx);

        let buffer = self.at(idx);
        self.ref_table[ref_cnt].push(idx);
        buffer
    }

    pub fn free(&mut self) -> Option<Vec<Buffer>> {
        let mut ret = Vec::new();
        for v in &self.ref_table {
            if v.len() == 0 {
                continue;
            }
            for idx in v {
                if !self.bitmap.is_true(*idx) {
                    let off = self.head.offset() + idx * (self.head_size as u64);
                    ret.push(self.head.allocate(off, self.head_size as u64))
                }
            }
            if ret.len() != 0 {
                return Some(ret);
            }
        }
        None
    }

    fn at(&mut self, idx: u64) -> Buffer {
        let off = self.head.offset() + idx * (self.head_size as u64);
        self.head.allocate(off, self.head_size as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut bytes = [0u8; 1024 * 17];
        let head_size = 16;
        let mut hs = HeadSegment::new(bytes.as_mut_ptr(), 1024, head_size);
        for i in 0..1024 {
            let head = hs.allocate(0);
            write_head(head.as_mut_ptr(), i, i as u32, true);
        }
        for i in 0..1024 {
            assert!(hs.bitmap.is_true(i as u64));
            let head = bytes[1024 + i * 16..1024 + i * 16 + 16].as_mut_ptr();
            let end = head_end(head);
            let off = head_off(head);
            let len = head_len(head);
            assert!(end);
            assert_eq!(off, i as u64);
            assert_eq!(len, i as u32);
        }
    }
}
