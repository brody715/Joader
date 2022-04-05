use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;

use super::joader::*;
use crate::proto::job::Data;
use crate::{
    local_cache::cache::Cache,
    new_dataset::new_dummy,
    job::Job,
    new_joader::joader_table::JoaderTable,
};

async fn write(mut jt: JoaderTable, len: usize) {
    let mut cnt = 0;
    loop {
        jt.next().await;
        cnt += 1;
        if cnt == len {
            break;
        }
    }
    assert_eq!(cnt, len);
}

async fn read(mut recv: Receiver<Arc<Vec<Data>>>, len: usize) -> Vec<Arc<Vec<Data>>> {
    let mut res = Vec::new();
    loop {
        let data = recv.recv().await;
        match data {
            Some(data) => res.push(data),
            None => continue,
        }
        if res.len() == len {
            break;
        }
    }
    assert_eq!(res.len(), len);
    res
}

#[tokio::test]
async fn test_joader_dummy() {
    // log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let cache = Arc::new(Mutex::new(Cache::new()));
    let mut jt = JoaderTable::new(cache);
    
    let len = 4096;
    let name = "dummy".to_string();
    let dataset = new_dummy(len, name.clone());
    let mut joader = Joader::new(dataset);
    let (job, recv) = Job::new(0);
    joader.add_job(job.clone()).await;
    jt.add_joader(joader);
    tokio::spawn(async move { write(jt, len).await });
    tokio::spawn(async move{ read(recv, len).await }).await.unwrap();
}

// #[tokio::test]
// async fn test_1_loader() {
//     let len = 10000;
//     let name = "dummy".to_string();
//     let dataset = new_dummy(len, name.clone());
//     let mut joader = Joader::new(dataset);
//     let (s, r) = create_data_channel(0);
//     joader.add_loader(0, 1);
//     joader.add_data_sender(0, s);
//     let cache = Arc::new(Mutex::new(Cache::new(256, &name, 1)));
//     tokio::spawn(async move { write(joader, cache).await });
//     let mut res = tokio::spawn(async move { read_data(r).await })
//         .await
//         .unwrap();
//     res.sort();
//     assert_eq!(res, (0..len).map(|x| x as u64).collect::<Vec<_>>());
// }

// #[tokio::test]
// async fn test_k_loader() {
//     let lengths = 4096;
//     let k = 8;
//     let name = "dummy".to_string();
//     let dataset = new_dummy(lengths, name.clone());
//     let mut joader = Joader::new(dataset);
//     let mut reader_map = HashMap::new();
//     for id in 0..k {
//         let (s, r) = create_data_channel(id as u64);
//         joader.add_loader(id as u64, 1);
//         joader.add_data_sender(id, s);
//         reader_map.insert(id, tokio::spawn(async move { read_data(r).await }));
//     }
//     let cache = Arc::new(Mutex::new(Cache::new(256, &name, 1)));
//     tokio::spawn(async move { write(joader, cache).await });

//     for (_id, handler) in reader_map.iter_mut() {
//         let mut res = handler.await.unwrap();
//         res.sort();
//         assert_eq!(res, (0..lengths).map(|x| x as u64).collect::<Vec<_>>());
//     }
// }

// #[tokio::test]
// async fn test_1_loader_k_sampler() {
//     let lengths = 2000;
//     let k = 6;
//     let name = "dummy".to_string();
//     let dataset = new_dummy(lengths, name.clone());
//     let mut joader = Joader::new(dataset);
//     let mut id_reader_map = HashMap::new();
//     let mut data_reader_map = HashMap::new();
//     joader.add_loader(0, 1);
//     let (s, r) = create_data_channel(0);
//     joader.add_data_sender(0, s);
//     joader.set_hash_key(k);
//     data_reader_map.insert(k, tokio::spawn(async move { read_data(r).await }));
//     for host_id in 0..k {
//         let (s, r) = create_idx_channel(0 as u64);
//         joader.add_idx_sender(0, s, host_id.into());
//         id_reader_map.insert(host_id, tokio::spawn(async move { read_indices(r).await }));
//     }
//     let cache = Arc::new(Mutex::new(Cache::new(256, &name, 1)));
//     tokio::spawn(async move { write(joader, cache).await });
//     let mut res = Vec::new();
//     for (id, handler) in id_reader_map.iter_mut() {
//         let mut indices = handler.await.unwrap();
//         println!("{} sample {} indices", id, indices.len());
//         res.append(&mut indices);
//     }
//     for (id, handler) in data_reader_map.iter_mut() {
//         let data = handler.await.unwrap();
//         println!("{} read {} data", id, data.len());
//         res.append(&mut data.iter().cloned().map(|x| x as u32).collect::<Vec<_>>());
//     }
//     res.sort();
//     assert_eq!(res, (0..lengths).map(|x| x as u32).collect::<Vec<_>>());
// }

// #[tokio::test]
// async fn test_k_loader_m_sampler() {
//     let lengths = 2000;
//     let k = 8;
//     let m = 8;
//     let name = "dummy".to_string();
//     let dataset = new_dummy(lengths, name.clone());
//     let mut joader = Joader::new(dataset);
//     joader.set_hash_key(m);
//     let mut id_reader_map = HashMap::new();
//     let mut data_reader_map = HashMap::new();
//     let mut res = HashMap::new();
//     for loader_id in 0..k {
//         res.insert(loader_id, Vec::new());
//     }
//     for loader_id in 0..k {
//         joader.add_loader(loader_id, 1);
//         let (s, r) = create_data_channel(loader_id);
//         joader.add_data_sender(loader_id, s);
//         data_reader_map.insert(loader_id, tokio::spawn(async move { read_data(r).await }));
//         for host_id in 0..m {
//             let (s, r) = create_idx_channel(loader_id as u64);
//             joader.add_idx_sender(loader_id, s, host_id.into());
//             id_reader_map.insert(
//                 (loader_id, host_id),
//                 tokio::spawn(async move { read_indices(r).await }),
//             );
//         }
//     }

//     let cache = Arc::new(Mutex::new(Cache::new(256, &name, 1)));
//     tokio::spawn(async move { write(joader, cache).await });

//     for ((loader_id, host_id), handler) in id_reader_map.iter_mut() {
//         let mut indices = handler.await.unwrap();
//         println!("{} {}sample {} indices", host_id, loader_id, indices.len());
//         res.get_mut(&loader_id).unwrap().append(&mut indices);
//     }

//     for (loader_id, handler) in data_reader_map.iter_mut() {
//         let data = handler.await.unwrap();
//         println!("{} read {} data", loader_id, data.len());
//         res.get_mut(loader_id)
//             .unwrap()
//             .append(&mut data.iter().cloned().map(|x| x as u32).collect::<Vec<_>>());
//     }

//     for (_, data) in res.iter_mut() {
//         data.sort();
//         assert_eq!(*data, (0..lengths).map(|x| x as u32).collect::<Vec<_>>());
//     }
// }
