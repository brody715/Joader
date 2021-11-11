use std::{thread, time};

use super::Buffer;

pub struct Bitmap {
    buf: Buffer
}

impl Bitmap {
    pub fn is_true(&self, idx: u64) -> bool {
        self.buf.get_idx(idx as isize) != 0
    }

    pub fn set(&mut self, idx: u64) {
        self.buf.set_idx(idx as isize, 1);
    }

    pub fn new(buf: Buffer) -> Bitmap {
        Bitmap { buf }
    }

    pub fn len(&self) -> u64 {
        self.buf.len()
    }

    //todo(xj): add free list
    pub fn find_free(&mut self) -> u64 {
        loop {
            for idx in 0..self.len() {
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
    use super::Bitmap;
    use super::*;
    #[test]
    fn test() {
        let mut buf = [0u8; 16];
        let mut bm = Bitmap::new(Buffer::new(buf.as_mut_ptr(), 0, 16));
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
