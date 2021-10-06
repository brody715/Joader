use crate::cache::bit::BitMap;
// head: |--end--|--len--|--addr--|
//       |--4--|--4--|--8--|
pub struct HeadMap {
    bitmap: BitMap,
    addr: *mut u8,
    len: usize,
    head_size: usize,
}

impl HeadMap {
    pub fn new(addr: *mut u8, len: usize, head_size: usize) -> (HeadMap, usize) {
        let size = len * head_size + len;
        unsafe {
            (
                HeadMap {
                    addr: addr.offset(len as isize),
                    len,
                    head_size,
                    bitmap: BitMap::new(addr, len),
                },
                size,
            )
        }
    }
    pub fn allocate_slot(&mut self) -> *mut u8 {
        unsafe { self.addr.offset(self.bitmap.find_free() as isize) }
    }
    pub fn write_head(addr: *mut u8, data_addr: usize, data_size: usize, end: bool) {
        unsafe {

            addr.offset(0).copy_from((end as u32).to_be_bytes().as_ptr(), 4);
            addr.offset(4).copy_from((data_size as u32).to_be_bytes().as_ptr(), 4);
            addr.offset(8).copy_from((data_addr as u64).to_be_bytes().as_ptr(), 8);
        }
    }
}
