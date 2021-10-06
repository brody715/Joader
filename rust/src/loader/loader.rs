use std::io::Read;
use std::{fs::File, io};

use crossbeam::channel::{Receiver, Sender};
use crate::dataset::{DataRequest, DatasetType};
use crate::cache::{self, Cache};

pub struct LoaderManager {
    cache: Cache
}

fn read_file(cache: &mut Cache, path: &str) {
    let mut f = File::open(path).expect("open file error");
    let mut block = cache.allocate();
    let read_bytes = 0;
    loop {
        let n = f.read(&mut block[read_bytes..]).expect("read error");
        read_bytes += n;
        if n == 0 {
            // reading the total file
            cache.finish(read_bytes, true);
            break;
        } else if n == block.len() {
            // use up block
            cache.finish(read_bytes, false);
            read_bytes = 0;
            block = cache.allocate();
        }
    }
}


impl LoaderManager {
    pub fn new() -> LoaderManager {todo!()}

    pub fn load(&mut self, req: DataRequest) {
        match req.dataset {
            DatasetType::FileSystem => self.load_file(&req.key.keys()),
            DatasetType::LMDB(_) => todo!(),
        }
    }
    
    pub fn load_file(&mut self, key: &[String]) {
        let cache = &mut self.cache;
        for path in key {
            read_file(cache, path);
        }
    }
}