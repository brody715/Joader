use crate::cache::Cache;
use crate::dataset::DataRequest;
use std::fs::File;
use std::io::Read;

pub struct LoaderManager {
    cache: Cache,
}

impl LoaderManager {
    pub fn new() -> LoaderManager {
        todo!()
    }

    pub fn load(&mut self, req: DataRequest) {
        // match req.dataset {
        //     DatasetType::FileSystem => self.load_file(&req.key.keys()),
        //     DatasetType::LMDB(_) => todo!(),
        // }
        todo!()
    }

    pub fn filesystem_load(&mut self, key: &[String]) {
        for path in key {
            let mut file = File::open(path).expect("open file error");
            let mut block = self.cache.next_block(None, 0);
            let size = file.read(block.as_mut_slice()).expect("reading data error");
            let mut remain_block = block.occupy(size);
            loop {
                // write flow:
                // allocate block -> write -> occupy(size)
                // if size < block, then some space remain
                // if size = block, then return None
                // if size == 0, then finish writing and free current block
                let mut last_block = block;
                if let Some(_b) = remain_block {
                    block = _b;
                } else {
                    block = self.cache.next_block(Some(last_block), 0);
                }
                let size = file.read(block.as_mut_slice()).expect("reading data error");
                // better way to solve the problem of size < block.size()
                remain_block = block.occupy(size);
                if size == 0 {
                    self.cache.free_block(block);
                    last_block.finish();
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    struct T {
        x: i32,
        y: i32,
    }
    #[test]
    fn test() {
        let mut t = T { x: 1, y: 2 };
    }
}
