use std::{thread, time};

pub struct BitMap {
    addr: *mut u8,
    len: usize,
}

impl BitMap {
    pub fn is_true(&mut self, idx: usize) -> bool {
        unsafe { *self.addr.offset(idx as isize) == 1 }
    }

    pub fn set(&mut self, idx: usize) {
        unsafe {
            *(self.addr.offset(idx as isize)) = 1;
        }
    }

    pub fn new(addr: *mut u8, len: usize) -> BitMap {
        BitMap { addr, len }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    //todo(xj): add free list
    pub fn find_free(&mut self) -> usize {
        loop {
            for idx in 0..self.len {
                if !self.is_true(idx) {
                    return idx;
                }
            }
            thread::sleep(time::Duration::from_secs_f32(0.01));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BitMap;

    #[test]
    fn test() {
        let mut buf = [0u8; 16];
        let mut bm = BitMap::new(buf.as_mut_ptr(), 16);
        for i in 0..bm.len() {
            if i%2 == 0 {
                bm.set(i)
            }
        }

        for i in 0..bm.len() {
            if i%2 == 0 {
                assert_eq!(bm.is_true(i), true)
            }
        }
    }
}
