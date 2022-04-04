use super::Dataset;
use super::DatasetRef;
use crate::process::MsgObject;
use crate::process::msg_unpack;
use crate::proto::job::Data;
use crate::proto::job::data::DataType;
use crate::proto::dataset::{CreateDatasetRequest, DataItem};
use lmdb::Database;
use lmdb::EnvironmentFlags;
use lmdb::Transaction;
use opencv::imgcodecs::imdecode;
use opencv::imgproc::cvt_color;
use opencv::imgproc::COLOR_BGR2RGB;
use opencv::prelude::Mat;
use opencv::prelude::MatTrait;
use opencv::prelude::MatTraitConst;
use std::path::Path;
use std::slice::from_raw_parts;
use std::{fmt::Debug, sync::Arc};
#[derive(Debug)]
struct LmdbDataset {
    items: Vec<DataItem>,
    id: u32,
    env: Arc<lmdb::Environment>,
    db: Database
}

pub fn from_proto(request: CreateDatasetRequest, id: u32) -> DatasetRef {
    let location = request.location;
    let items = request.items;
    let p = Path::new(&location);
    let env = lmdb::Environment::new()
        .set_flags(
            EnvironmentFlags::NO_SUB_DIR
                | EnvironmentFlags::READ_ONLY
                | EnvironmentFlags::NO_MEM_INIT
                | EnvironmentFlags::NO_LOCK
                | EnvironmentFlags::NO_SYNC,
        )
        .open_with_permissions(p, 0o600)
        .unwrap();
    Arc::new(LmdbDataset {
        items,
        id,
        db: env.open_db(None).unwrap(),
        env: Arc::new(env),
    })
}

#[inline]
fn decode<'a>(data: &'a [u8]) -> (u64, Mat) {
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
    let data = match *content.as_ref() {
        MsgObject::Bin(bin) => bin,
        _ => unimplemented!(),
    };
    let mat = Mat::from_slice(data).unwrap();
    let image = imdecode(&mat, 1).unwrap();
    (label, image)
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

    fn read(&self, idx: u32) -> Arc<Vec<Data>> {
        let txn = self.env.begin_ro_txn().unwrap();
        let key = self.items[idx as usize].keys[0].clone();
        let data: &[u8] = txn.get(self.db, &key.to_string()).unwrap();
        let (label, image) = decode(data.as_ref());

        let mut dst = Mat::default();
        cvt_color(&image, &mut dst, COLOR_BGR2RGB, 0).unwrap();
        let h = image.rows();
        let w = image.cols();
        let img_size = (h * w * image.channels()) as usize;
        let mut data = unsafe { 
            let raw = dst.data_mut();
            from_raw_parts(raw, img_size).to_vec()
        };
        let mut h = h.to_be_bytes().to_vec();
        let mut w = w.to_be_bytes().to_vec();
        data.append(&mut h);
        data.append(&mut w);
        data.push(image.channels() as u8);
        
        let label = Data {
            bs: label.to_be_bytes().to_vec(),
            ty: DataType::Uint64 as i32,
        };
        let data = Data {
            bs: data,
            ty: DataType::Image as i32
        };
        Arc::new(vec![label, data])
    }

    fn len(&self) -> u64 {
        self.items.len() as u64
    }
}

// #[cfg(test)]
// mod tests {
//     use lmdb::Transaction;
//     use tokio::join;

//     use super::*;
//     use crate::cache::head::Head;
//     use crate::joader::joader::Joader;
//     use crate::job::create_data_channel;
//     use std::process::Command;
//     use std::time::SystemTime;

//     #[test]
//     fn test_read_bacth() {
//         println!(
//             "{:?} {:?}",
//             Command::new("dd")
//                 .arg("if=/home/xiej/nfs/lmdb-imagenet/ILSVRC-train.lmdb")
//                 .arg("iflag=nocache")
//                 .arg("count=0")
//                 .output()
//                 .expect("cmd exec error!"),
//             Command::new("dd")
//                 .arg("if=/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb")
//                 .arg("iflag=nocache")
//                 .arg("count=0")
//                 .output()
//                 .expect("cmd exec error!")
//         );
//         log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
//         let location = "/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb".to_string();
//         let len = 32 * 128;
//         let name = "DLCache".to_string();
//         let cache = Arc::new(Mutex::new(Cache::new(2048 * 1024 * 1024, &name, 2048)));
//         let mut items = Vec::new();
//         for i in 0..len as usize {
//             items.push(DataItem {
//                 keys: vec![i.to_string()],
//             })
//         }
//         let p = Path::new(&location);
//         let env = lmdb::Environment::new()
//             .set_flags(
//                 EnvironmentFlags::NO_SUB_DIR
//                     | EnvironmentFlags::READ_ONLY
//                     | EnvironmentFlags::NO_MEM_INIT
//                     | EnvironmentFlags::NO_LOCK
//                     | EnvironmentFlags::NO_SYNC,
//             )
//             .open_with_permissions(p, 0o600)
//             .unwrap();
//         let dataset = Arc::new(LmdbDataset {
//             items,
//             id: 0,
//             db: env.open_db(None).unwrap(),
//             env: Arc::new(env),
//             pool: Arc::new(Mutex::new(ThreadPool::new(POOL_SIZE))),
//         });
//         let now = SystemTime::now();
//         let batch_size = POOL_SIZE as usize;

//         for i in 0..(len / batch_size as usize) {
//             let mut batch_data = HashMap::new();
//             for idx in i..(i + batch_size) {
//                 batch_data.insert(idx as u32, (0usize, 1usize));
//             }
//             dataset.read_batch(cache.clone(), batch_data);
//         }
//         let time = SystemTime::now().duration_since(now).unwrap().as_secs_f32();
//         println!("total{} avg{}", time, time / (len as f32));
//     }
//     #[test]
//     fn test_read_decode_bacth() {
//         println!(
//             "{:?} {:?}",
//             Command::new("dd")
//                 .arg("if=/home/xiej/nfs/lmdb-imagenet/ILSVRC-train.lmdb")
//                 .arg("iflag=nocache")
//                 .arg("count=0")
//                 .output()
//                 .expect("cmd exec error!"),
//             Command::new("dd")
//                 .arg("if=/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb")
//                 .arg("iflag=nocache")
//                 .arg("count=0")
//                 .output()
//                 .expect("cmd exec error!")
//         );
//         log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
//         let location = "/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb".to_string();
//         let len = 32 * 128;
//         let name = "DLCache".to_string();
//         let cache = Arc::new(Mutex::new(Cache::new(2048 * 1024 * 1024, &name, 2048)));
//         let mut items = Vec::new();
//         for i in 0..len as usize {
//             items.push(DataItem {
//                 keys: vec![i.to_string()],
//             })
//         }
//         let p = Path::new(&location);
//         let env = lmdb::Environment::new()
//             .set_flags(
//                 EnvironmentFlags::NO_SUB_DIR
//                     | EnvironmentFlags::READ_ONLY
//                     | EnvironmentFlags::NO_MEM_INIT
//                     | EnvironmentFlags::NO_LOCK
//                     | EnvironmentFlags::NO_SYNC,
//             )
//             .open_with_permissions(p, 0o600)
//             .unwrap();
//         let dataset = Arc::new(LmdbDataset {
//             items,
//             id: 0,
//             db: env.open_db(None).unwrap(),
//             env: Arc::new(env),
//             pool: Arc::new(Mutex::new(ThreadPool::new(POOL_SIZE))),
//         });
//         let now = SystemTime::now();
//         let batch_size = POOL_SIZE as usize;

//         for i in 0..(len / batch_size as usize) {
//             let mut batch_data = HashMap::new();
//             for idx in i..(i + batch_size) {
//                 batch_data.insert(idx as u32, (0usize, 1usize));
//             }
//             dataset.read_decode_batch(cache.clone(), batch_data);
//         }
//         let time = SystemTime::now().duration_since(now).unwrap().as_secs_f32();
//         println!("total{} avg{}", time, time / (len as f32));
//     }
//     #[test]
//     fn test_decode() {
//         log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
//         let location = "/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb".to_string();
//         let len = 1001;
//         let name = "DLCache".to_string();
//         let cache = Arc::new(Mutex::new(Cache::new(1024 * 1024 * 1024, &name, 1024)));
//         let mut items = Vec::new();
//         for i in 0..len as usize {
//             items.push(DataItem {
//                 keys: vec![i.to_string()],
//             })
//         }
//         let p = Path::new(&location);
//         let env = lmdb::Environment::new()
//             .set_flags(EnvironmentFlags::NO_SUB_DIR | EnvironmentFlags::READ_ONLY)
//             .open_with_permissions(p, 0o600)
//             .unwrap();
//         let dataset = Arc::new(LmdbDataset {
//             items,
//             id: 0,
//             db: env.open_db(None).unwrap(),
//             env: Arc::new(env),
//             pool: Arc::new(Mutex::new(ThreadPool::new(POOL_SIZE))),
//         });
//         let mut batch_data = HashMap::new();
//         batch_data.insert(10u32, (0usize, 1usize));
//         batch_data.insert(20u32, (0usize, 1usize));
//         batch_data.insert(30u32, (0usize, 1usize));
//         batch_data.insert(40u32, (0usize, 1usize));
//         batch_data.insert(50u32, (0usize, 1usize));
//         dataset.read_decode_batch(cache, batch_data);
//     }
//     #[test]
//     fn test_lmdb() {
//         println!(
//             "{:?} {:?}",
//             Command::new("dd")
//                 .arg("if=/home/xiej/nfs/lmdb-imagenet/ILSVRC-train.lmdb")
//                 .arg("iflag=nocache")
//                 .arg("count=0")
//                 .output()
//                 .expect("cmd exec error!"),
//             Command::new("dd")
//                 .arg("if=/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb")
//                 .arg("iflag=nocache")
//                 .arg("count=0")
//                 .output()
//                 .expect("cmd exec error!")
//         );
//         let location = "/home/xiej/nfs/lmdb-imagenet/ILSVRC-train.lmdb";
//         let p = Path::new(&location);
//         let env = lmdb::Environment::new()
//             .set_flags(
//                 EnvironmentFlags::NO_SUB_DIR
//                     | EnvironmentFlags::READ_ONLY
//                     | EnvironmentFlags::NO_META_SYNC
//                     | EnvironmentFlags::NO_SYNC
//                     | EnvironmentFlags::NO_MEM_INIT
//                     | EnvironmentFlags::NO_LOCK
//                     | EnvironmentFlags::NO_READAHEAD,
//             )
//             .set_max_readers(1024)
//             .open_with_permissions(p, 0o400)
//             .unwrap();
//         let now = SystemTime::now();
//         let len = 10000;
//         let db = env.open_db(None).unwrap();
//         let txn = env.begin_ro_txn().unwrap();
//         for i in 0..len {
//             txn.get(db, &(i.to_string())).unwrap();
//             if i != 0 && i % 100 == 0 {
//                 let time = SystemTime::now().duration_since(now).unwrap().as_secs_f32();
//                 print!("read {} data need {}, avg: {}\n", i, time, time / i as f32);
//             }
//         }
//     }

//     #[tokio::test]
//     async fn test_cache_lmdb() {
//         let batch_size = POOL_SIZE;
//         println!(
//             "{:?} {:?}",
//             Command::new("dd")
//                 .arg("if=/home/xiej/nfs/lmdb-imagenet/ILSVRC-train.lmdb")
//                 .arg("iflag=nocache")
//                 .arg("count=0")
//                 .output()
//                 .expect("cmd exec error!"),
//             Command::new("dd")
//                 .arg("if=/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb")
//                 .arg("iflag=nocache")
//                 .arg("count=0")
//                 .output()
//                 .expect("cmd exec error!")
//         );
//         let location = "/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb".to_string();
//         let len = 1281166;
//         let name = "DLCache".to_string();
//         let cache = Arc::new(Mutex::new(Cache::new(1024 * 1024 * 1024, &name, 1024)));
//         let mut items = Vec::new();
//         for i in 0..len as usize {
//             items.push(DataItem {
//                 keys: vec![i.to_string()],
//             })
//         }
//         let p = Path::new(&location);
//         let env = lmdb::Environment::new()
//             .set_flags(
//                 EnvironmentFlags::NO_SUB_DIR
//                     | EnvironmentFlags::READ_ONLY
//                     | EnvironmentFlags::NO_MEM_INIT
//                     | EnvironmentFlags::NO_LOCK
//                     | EnvironmentFlags::NO_SYNC,
//             )
//             .open_with_permissions(p, 0o600)
//             .unwrap();
//         let dataset = Arc::new(LmdbDataset {
//             items,
//             id: 0,
//             db: env.open_db(None).unwrap(),
//             env: Arc::new(env),
//             pool: Arc::new(Mutex::new(ThreadPool::new(POOL_SIZE))),
//         });
//         let mut joader = Joader::new(dataset);
//         let (s, mut r) = create_data_channel(0);
//         joader.add_loader(0, 1);
//         joader.add_data_sender(0, s);
//         let thread_cache = cache.clone();
//         let reader = tokio::spawn(async move {
//             let now = SystemTime::now();
//             let mut consume = 0;
//             loop {
//                 let (indices, empty) = r.recv_all().await;
//                 {
//                     let start_ptr = thread_cache.lock().unwrap().start_ptr();
//                     for idx in &indices {
//                         let addr =
//                             unsafe { start_ptr.offset((*idx as isize) * (Head::size() as isize)) };
//                         let mut head = Head::from(addr);
//                         head.readed(1);
//                     }
//                 }
//                 consume += indices.len();
//                 if consume != 0 && consume % 1000 == 0 {
//                     let time = SystemTime::now().duration_since(now).unwrap().as_secs_f32();
//                     print!(
//                         "read {} data need {}, avg: {}\n",
//                         consume,
//                         time,
//                         time / consume as f32
//                     );
//                 }
//                 if consume == len || empty {
//                     break;
//                 }
//             }
//             println!("exist reading.....");
//         });

//         let writer = tokio::spawn(async move {
//             let now = SystemTime::now();
//             for i in 0..(len / batch_size) as usize {
//                 // println!("{:}", i);
//                 joader.next_batch(cache.clone(), batch_size).await;
//                 let time = SystemTime::now().duration_since(now).unwrap().as_secs_f32();
//                 if i != 0 && (i * batch_size) % 1000 == 0 {
//                     print!(
//                         "write {} data need {}, avg: {}\n",
//                         i * batch_size,
//                         time,
//                         time / ((i * batch_size) as f32)
//                     );
//                 }
//             }
//         });
//         let res = join!(reader, writer);
//         res.0.unwrap();
//         res.1.unwrap();
//     }
// }
