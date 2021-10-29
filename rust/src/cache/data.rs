use super::{Buffer, FreeList};

pub struct DataSegment {
    data: Buffer,
    free_list: FreeList
}

impl DataSegment {
    pub fn new(ptr: *mut u8, offset: usize, len: usize) -> DataSegment {
        todo!()
    }
    pub fn allocate(&mut self) -> Option<Buffer> {todo!()}
}