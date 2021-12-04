use super::Dataset;
use super::DatasetRef;
use crate::{
    cache::cache::Cache,
    proto::dataset::{CreateDatasetRequest, DataItem},
};
use lmdb::open::{NOSUBDIR, RDONLY};
use lmdb::Database;
use lmdb::ReadTransaction;
use lmdb_zero as lmdb;
use std::{cmp::min, fmt::Debug, sync::Arc};

#[derive(Debug)]
struct LmdbDataset {
    magic: u8,
    items: Vec<DataItem>,
    location: String,
    name: String,
    env: lmdb::Environment,
}

pub fn from_proto(request: CreateDatasetRequest) -> DatasetRef {
    let name = request.name;
    let location = request.location;
    let items = request.items;
    let env = unsafe {
        lmdb::EnvBuilder::new()
            .unwrap()
            .open(&location, RDONLY | NOSUBDIR, 0o600)
            .unwrap()
    };
    // Open the default database.
    Arc::new(LmdbDataset {
        items,
        magic: 7u8,
        name,
        location,
        env,
    })
}

impl LmdbDataset {
    fn read_one(
        &self,
        cache: &mut Cache,
        db: &Database,
        txn: &ReadTransaction,
        key: &str,
        ref_cnt: usize,
    ) -> u64 {
        let acc = txn.access();
        let mut data: &[u8] = acc.get(db, key).unwrap();
        let mut len = data.len();

        let data_name = self.name.clone() + key;
        if let Some(addr) = cache.contains_data(&data_name) {
            return addr as u64;
        }

        let (mut block, idx) = cache.next_block(None, ref_cnt, &data_name);
        let mut block_slice = block.as_mut_slice();
        let mut write_size = min(len, block_slice.len());

        block_slice[..write_size].copy_from_slice(&data[..write_size]);
        data = &data[write_size..];

        let mut remain_block = block.occupy(write_size as usize);
        len -= write_size;
        loop {
            let mut last_block = block;
            // write flow:
            // allocate block -> write -> occupy(size)
            // if size < block, then some space remain
            // if size = block, then return None
            // if size == 0, then finish writing and free current block
            if let Some(_b) = remain_block {
                block = _b;
            } else {
                block = cache.next_block(Some(last_block), 0, &data_name).0;
            }
            block_slice = block.as_mut_slice();
            write_size = min(len, block_slice.len());

            block_slice[..write_size].copy_from_slice(&data[..write_size]);
            data = &data[write_size..];
            remain_block = block.occupy(write_size as usize);
            len -= write_size;
            if write_size == 0 {
                cache.free_block(block);
                last_block.finish();
                break;
            }
        }
        idx as u64
    }
}

impl Dataset for LmdbDataset {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_indices(&self) -> Vec<u32> {
        let start = 0u32;
        let end = self.items.len() as u32;
        (start..end).collect::<Vec<_>>()
    }

    fn read(&self, cache: &mut Cache, idx: u32, ref_cnt: usize) -> u64 {
        let db = lmdb::Database::open(&self.env, None, &lmdb::DatabaseOptions::defaults()).unwrap();
        let txn = lmdb::ReadTransaction::new(&self.env).unwrap();
        let key = &self.items[idx as usize].keys[0];
        self.read_one(cache, &db, &txn, key, ref_cnt)
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;
    #[test]
    fn test_lmdb() {
        let location = "/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb";
        let env = unsafe {
            lmdb::EnvBuilder::new()
                .unwrap()
                .open(location, RDONLY | NOSUBDIR, 0o600)
                .unwrap()
        };

        let now = SystemTime::now();
        let len = 10000;
        for i in 0..len {
            let db = lmdb::Database::open(&env, None, &lmdb::DatabaseOptions::defaults()).unwrap();
            let txn = lmdb::ReadTransaction::new(&env).unwrap();
            let acc = txn.access();
            let data: &[u8] = acc.get(&db, i.to_string().as_bytes()).unwrap();
            println!("{}", data.len());
        }
        let time = SystemTime::now().duration_since(now).unwrap().as_secs_f32();
        print!("{}, avg: {}", time, time / len as f32);
    }
}
