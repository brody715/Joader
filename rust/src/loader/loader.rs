use std::io::Read;
use std::{fs::File, io};

use crate::cache::{self, Cache};
use crate::dataset::{DataRequest, DatasetType};
use crossbeam::channel::{Receiver, Sender};

pub struct LoaderManager {
    cache: Cache,
}

impl LoaderManager {
    pub fn new() -> LoaderManager {
        todo!()
    }

    pub fn load(&mut self, req: DataRequest) {
        match req.dataset {
            DatasetType::FileSystem => self.load_file(&req.key.keys()),
            DatasetType::LMDB(_) => todo!(),
        }
    }

    pub fn load_file(&mut self, key: &[String]) {
        let cache = &mut self.cache;
        let mut block = cache.next_block(vec![0u8].as_slice(), false).unwrap();
        for path in key {
            let mut f = File::open(path).expect("open file error");
            let mut read_bytes = 0;
            loop {
                let n = f.read(&mut block[read_bytes..]).expect("read error");
                read_bytes += n;
                if n == 0 {
                    // reading the total file
                    cache.next_block(&block[..read_bytes], true);
                    block = &mut block[read_bytes..];
                    break;
                } else if n == block.len() {
                    // use up block
                    block = cache.next_block(&block[..read_bytes], false).unwrap();
                    read_bytes = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    struct T{x: i32, y: i32}
    #[test]
    fn test () {
        let mut t = T{x:1, y:2};
        let x = &mut t.x;
        let c = &mut t;
        *x = 1;
    }
}
