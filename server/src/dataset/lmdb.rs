use super::data_id;
use super::Dataset;
use super::DatasetRef;
use crate::process::get_array_head_size;
use crate::process::get_bin_size_from_len;
use crate::process::get_int_size;
use crate::process::msg_unpack;
use crate::process::MsgObject;
use crate::{
    cache::cache::Cache,
    proto::dataset::{CreateDatasetRequest, DataItem},
};
use crossbeam;
use image::jpeg::JpegDecoder;
use image::ImageDecoder;
use lmdb::open::{NOSUBDIR, RDONLY};
use lmdb::Database;
use lmdb::Environment;
use lmdb_zero as lmdb;
use rmp::encode::write_array_len;
use rmp::encode::write_bin_len;
use rmp::encode::write_uint;
use std::io::Cursor;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
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

#[inline]
fn decode<'a>(data: &'a [u8]) -> (u64, JpegDecoder<Cursor<&'a [u8]>>) {
    let data = msg_unpack(data);
    let data = match &data[0] {
        MsgObject::Array(data) => data,
        _ => unimplemented!(),
    };
    let image = &data[0];
    let label = match data[1].as_ref() {
        &MsgObject::UInt(b) => b,
        _ => unimplemented!(),
    };
    let content = match image.as_ref() {
        MsgObject::Map(map) => &map["data"],
        err => unimplemented!("{:?}", err),
    };
    let decoder = match *content.as_ref() {
        MsgObject::Bin(bin) => JpegDecoder::new(Cursor::new(bin)).unwrap(),
        _ => unimplemented!(),
    };
    (label, decoder)
}

fn read_decode_one(
    id: u64,
    ref_cnt: usize,
    loader_cnt: usize,
    cache: Arc<Mutex<Cache>>,
    sender: Sender<u64>,
    db: &Database,
    env: &Environment,
    key: &str,
) {
    let txn = lmdb::ReadTransaction::new(env).unwrap();
    let acc = txn.access();
    let data: &[u8] = acc.get(db, key).unwrap();
    let (label, decoder) = decode(data.as_ref());
    let img_size = decoder.total_bytes();
    let (w, h) = decoder.dimensions();
    // |array [label, w, h, image]
    let array_size = 4;
    let array_head = get_array_head_size(array_size);

    let label_len = get_int_size(label);
    let width_len = get_int_size(w as u64);
    let height_len = get_int_size(h as u64);
    let bin_len = get_bin_size_from_len(img_size as usize);

    let len = array_head + bin_len + label_len + width_len + height_len;
    let (block_slice, idx) = {
        let mut locked_cache = cache.lock().unwrap();
        locked_cache.allocate(len, ref_cnt, id, loader_cnt)
    };
    assert_eq!(block_slice.len(), len);
    let mut writer = Cursor::new(block_slice);
    write_array_len(&mut writer, array_size as u32).unwrap();
    write_uint(&mut writer, label).unwrap();
    write_uint(&mut writer, w as u64).unwrap();
    write_uint(&mut writer, h as u64).unwrap();
    write_bin_len(&mut writer, img_size as u32).unwrap();
    decoder
        .read_image(&mut writer.into_inner()[len - img_size as usize..])
        .unwrap();
    log::debug!("Read and decode data {:?} at {:?} in lmdb", id, idx);
    sender.send(idx as u64).unwrap();
}

fn read_one(
    id: u64,
    ref_cnt: usize,
    loader_cnt: usize,
    cache: Arc<Mutex<Cache>>,
    sender: Sender<u64>,
    db: &Database,
    env: &Environment,
    key: &str,
) {
    let txn = lmdb::ReadTransaction::new(env).unwrap();
    let acc = txn.access();
    let data: &[u8] = acc.get(db, key).unwrap();
    let len = data.len();
    let (block_slice, idx) = {
        let mut cache = cache.lock().unwrap();
        cache.allocate(len, ref_cnt, id, loader_cnt)
    };
    assert_eq!(block_slice.len(), len);
    block_slice.copy_from_slice(data);
    log::debug!("Read data {:?} at {:?} in lmdb", id, idx);
    sender.send(idx as u64).unwrap();
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

    fn read_batch(
        &self,
        cache: Arc<Mutex<Cache>>,
        idx: Vec<u32>,
        ref_cnt: Vec<usize>,
        loader_cnt: Vec<usize>,
    ) -> Vec<u64> {
        let db = lmdb::Database::open(&self.env, None, &lmdb::DatabaseOptions::defaults()).unwrap();
        let mut ret = Vec::new();
        let (sender, receiver) = std::sync::mpsc::channel::<u64>();
        let mut producer_num = 0;
        crossbeam::scope(|s| {
            for ((&idx, &ref_cnt), &loader_cnt) in
                idx.iter().zip(ref_cnt.iter()).zip(loader_cnt.iter())
            {
                let data_id = data_id(self.id, idx);
                {
                    let mut cache = cache.lock().unwrap();
                    if let Some(head_idx) = cache.contains_data(data_id) {
                        cache.mark_unreaded(head_idx, loader_cnt);
                        log::debug!("Hit data {:?} at {:?} in lmdb", idx, head_idx);
                        ret.push(head_idx as u64);
                        continue;
                    }
                }
                let thread_cache = cache.clone();
                let thread_sender = sender.clone();
                let key = self.items[idx as usize].keys[0].as_str();
                let db_ref = &db;
                s.spawn(move |_| {
                    read_one(
                        data_id,
                        ref_cnt,
                        loader_cnt,
                        thread_cache,
                        thread_sender,
                        db_ref,
                        &self.env,
                        key,
                    )
                });
                producer_num += 1;
            }
            for _ in 0..producer_num {
                let idx = receiver.recv().unwrap();
                ret.push(idx);
            }
        })
        .unwrap();
        ret
    }

    fn read_decode_batch(
        &self,
        cache: Arc<Mutex<Cache>>,
        idx: Vec<u32>,
        ref_cnt: Vec<usize>,
        loader_cnt: Vec<usize>,
    ) -> Vec<u64> {
        let db = lmdb::Database::open(&self.env, None, &lmdb::DatabaseOptions::defaults()).unwrap();
        let mut ret = Vec::new();
        let (sender, receiver) = std::sync::mpsc::channel::<u64>();
        let mut producer_num = 0;
        crossbeam::scope(|s| {
            for ((&idx, &ref_cnt), &loader_cnt) in
                idx.iter().zip(ref_cnt.iter()).zip(loader_cnt.iter())
            {
                let data_id = data_id(self.id, idx);
                {
                    let mut cache = cache.lock().unwrap();
                    if let Some(head_idx) = cache.contains_data(data_id) {
                        cache.mark_unreaded(head_idx, loader_cnt);
                        log::debug!("Hit data {:?} at {:?} in lmdb", idx, head_idx);
                        ret.push(head_idx as u64);
                        continue;
                    }
                }
                producer_num += 1;
                let thread_cache = cache.clone();
                let thread_sender = sender.clone();
                let key = self.items[idx as usize].keys[0].as_str();
                let db_ref = &db;
                s.spawn(move |_| {
                    read_decode_one(
                        data_id,
                        ref_cnt,
                        loader_cnt,
                        thread_cache,
                        thread_sender,
                        db_ref,
                        &self.env,
                        key,
                    )
                });
            }
            for _ in 0..producer_num {
                let idx = receiver.recv().unwrap();
                ret.push(idx);
            }
        })
        .unwrap();
        ret
    }

    fn len(&self) -> u64 {
        self.items.len() as u64
    }
}

#[cfg(test)]
mod tests {
    use tokio::join;

    use super::*;
    use crate::cache::head::Head;
    use crate::joader::joader::Joader;
    use crate::loader::create_data_channel;
    use std::process::Command;
    use std::time::SystemTime;

    #[test]
    fn test_read_bacth() {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        let location = "/home/nfs/data/lmdb-imagenet/ILSVRC-train.lmdb".to_string();
        let len = 32 * 64;
        let name = "DLCache".to_string();
        let cache = Arc::new(Mutex::new(Cache::new(3096 * 1024 * 1024, &name, 2048)));
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
        let now = SystemTime::now();
        let batch_size = 8 as usize;
        for i in 0..(len / batch_size as usize) {
            dataset.read_batch(
                cache.clone(),
                (i..(i + batch_size)).map(|x| x as u32).collect::<Vec<_>>(),
                vec![0; batch_size],
                vec![1; batch_size],
            );
        }
        let time = SystemTime::now().duration_since(now).unwrap().as_secs_f32();
        println!("total{} avg{}", time, time / (len as f32));
    }
    #[test]
    fn test_decode() {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        let location = "/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb".to_string();
        let len = 1001;
        let name = "DLCache".to_string();
        let cache = Arc::new(Mutex::new(Cache::new(1024 * 1024 * 1024, &name, 1024)));
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
        dataset.read_batch(cache, vec![0], vec![0], vec![1]);
    }
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
        println!(
            "{:?} {:?}",
            Command::new("dd")
                .arg("if=/home/xiej/nfs/lmdb-imagenet/ILSVRC-train.lmdb")
                .arg("iflag=nocache")
                .arg("count=0")
                .output()
                .expect("cmd exec error!"),
            Command::new("dd")
                .arg("if=/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb")
                .arg("iflag=nocache")
                .arg("count=0")
                .output()
                .expect("cmd exec error!")
        );
        let location = "/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb".to_string();
        let len = 100000;
        let name = "DLCache".to_string();
        let cache = Arc::new(Mutex::new(Cache::new(1024 * 1024 * 1024, &name, 1024)));
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
        let thread_cache = cache.clone();
        let reader = tokio::spawn(async move {
            let now = SystemTime::now();
            let mut consume = 0;
            loop {
                let (indices, empty) = r.recv_all().await;
                {
                    let start_ptr = thread_cache.lock().unwrap().start_ptr();
                    for idx in &indices {
                        let addr =
                            unsafe { start_ptr.offset((*idx as isize) * (Head::size() as isize)) };
                        let mut head = Head::from(addr);
                        head.readed(1);
                    }
                }
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
                joader.next_batch(cache.clone(), 4).await;
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
