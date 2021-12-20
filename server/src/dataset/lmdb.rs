use super::data_id;
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
use std::{fmt::Debug, sync::Arc};

#[derive(Debug)]
struct LmdbDataset {
    items: Vec<DataItem>,
    id: u32,
    env: lmdb::Environment,
}

pub fn from_proto(request: CreateDatasetRequest, id: u32) -> DatasetRef {
    let location = request.location;
    let items = request.items;
    let env = unsafe {
        lmdb::EnvBuilder::new()
            .unwrap()
            .open(&location, RDONLY | NOSUBDIR, 0o600)
            .unwrap()
    };
    // Open the default database.
    Arc::new(LmdbDataset { items, id, env })
}

impl LmdbDataset {
    fn read_one(
        &self,
        cache: &mut Cache,
        db: &Database,
        txn: &ReadTransaction,
        id: u64,
        key: &str,
        ref_cnt: usize,
        loader_cnt: usize
    ) -> u64 {
        let acc = txn.access();
        let data: &[u8] = acc.get(db, key).unwrap();
        let len = data.len();
        let (block_slice, idx) = cache.allocate(len, ref_cnt, id, loader_cnt);
        assert_eq!(block_slice.len(), len);
        block_slice.copy_from_slice(data);
        idx as u64
    }
}

impl Dataset for LmdbDataset {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_indices(&self) -> Vec<u32> {
        let start = 0u32;
        let end = self.items.len() as u32;
        (start..end).collect::<Vec<_>>()
    }

    fn read(&self, cache: &mut Cache, idx: u32, ref_cnt: usize, loader_cnt: usize) -> u64 {
        let data_id = data_id(self.id, idx);
        if let Some(head_idx) = cache.contains_data(data_id) {
            cache.mark_unreaded(head_idx, loader_cnt);
            return head_idx as u64;
        }

        let db = lmdb::Database::open(&self.env, None, &lmdb::DatabaseOptions::defaults()).unwrap();
        let txn = lmdb::ReadTransaction::new(&self.env).unwrap();
        let key = &self.items[idx as usize].keys[0];
        self.read_one(cache, &db, &txn, data_id, key, ref_cnt, loader_cnt)
    }

    fn len(&self) -> u64 {
        self.items.len() as u64
    }
}

#[cfg(test)]
mod tests {
    use tokio::join;

    use super::*;
    use crate::joader::joader::Joader;
    use crate::loader::create_data_channel;
    use std::time::SystemTime;
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
            // cloned
            let _cloned_dat = data.to_vec();
            if len != 0 && len % 1000 == 0 {
                let time = SystemTime::now().duration_since(now).unwrap().as_secs_f32();
                print!(
                    "read {} data need {}, avg: {}\n",
                    len,
                    time,
                    time / len as f32
                );
            }
            println!("{}", data.len());
        }
    }

    #[tokio::test]
    async fn test_cache_lmdb() {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        let location = "/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb".to_string();
        let len = 1001;
        let name = "DLCache".to_string();
        let mut cache = Cache::new(1024 * 1024 * 1024, &name, 1024);
        let mut items = Vec::new();
        for i in 0..len as usize {
            items.push(DataItem {
                keys: vec![i.to_string()],
            })
        }
        let dataset = Arc::new(LmdbDataset {
            items,
            id: 0,
            env: unsafe {
                lmdb::EnvBuilder::new()
                    .unwrap()
                    .open(&location, RDONLY | NOSUBDIR, 0o600)
                    .unwrap()
            },
        });
        let mut joader = Joader::new(dataset);
        let (s, mut r) = create_data_channel(0);
        joader.add_loader(0, 1);
        joader.add_data_sender(0, s);

        let reader = tokio::spawn(async move {
            let now = SystemTime::now();
            let mut consume = 0;
            loop {
                let (indices, empty) = r.recv_all().await;
                consume += indices.len();
                if consume != 0 && consume % 1000 == 0 {
                    let time = SystemTime::now().duration_since(now).unwrap().as_secs_f32();
                    print!(
                        "read {} data need {}, avg: {}\n",
                        consume,
                        time,
                        time / consume as f32
                    );
                }
                if consume == len || empty {
                    break;
                }
            }
            println!("exist reading.....");
        });
        let writer = tokio::spawn(async move {
            let now = SystemTime::now();
            for i in 0..len {
                joader.next(&mut cache).await;
                let time = SystemTime::now().duration_since(now).unwrap().as_secs_f32();
                if i != 0 && i % 1000 == 0 {
                    print!("write {} data need {}, avg: {}\n", i, time, time / i as f32);
                }
            }
        });
        let res = join!(reader, writer);
        res.0.unwrap();
        res.1.unwrap();
    }
}
