use std::{io::BufRead, slice::from_raw_parts};

use crate::cache::bit::Bitmap;

use super::Buffer;
// head: |--end--|--len--|--addr--|
//       |--4--|--4--|--8--|
pub struct HeadSegment {
    bitmap: Bitmap,
    head: Buffer,
    // 16
    head_size: usize,
}

impl HeadSegment {
    pub fn new(addr: *mut u8, len: usize, head_size: usize) -> (HeadSegment, usize) {
        let size = len * head_size + len;
        unsafe {
            (
                HeadSegment {
                    bitmap: Bitmap::new(Buffer::new(addr, len)),
                    head: Buffer::new(addr.offset(len as isize), len*head_size),
                    head_size,
                },
                size,
            )
        }
    }
    pub fn allocate(&mut self) -> Buffer {
        todo!()
    }
    pub fn write_head(head: Buffer, data: Buffer, end: bool) {
        todo!()
    }
    pub fn replace_head(block: Buffer, next_block: Buffer) ->(Buffer, Buffer) {
        todo!()
    }
}
