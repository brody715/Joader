use std::collections::HashMap;

use super::joader::*;
use crate::{
    cache::cache::Cache,
    dataset::new_dummy,
    loader::{create_data_channel, create_idx_channel, DataReceiver, IdxReceiver},
};

async fn read_data(mut r: DataReceiver) -> Vec<u64> {
    let mut res = Vec::new();
    loop {
        let (mut indices, empty) = r.recv_all().await;
        res.append(&mut indices);
        if empty {
            r.close();
            break;
        }
    }
    return res;
}

async fn read_indices(mut r: IdxReceiver) -> Vec<u32> {
    let mut res = Vec::new();
    loop {
        let (mut indices, empty) = r.recv_all().await;
        res.append(&mut indices);
        if empty {
            r.close();
            break;
        }
    }
    return res;
}

async fn write(mut joader: Joader, mut cache: Cache) {
    loop {
        joader.next(&mut cache).await;
        if joader.is_empty() {
            break;
        }
    }
}

#[tokio::test]
async fn test_1_loader() {
    let len = 10000;
    let name = "dummy".to_string();
    let dataset = new_dummy(len, name.clone());
    let mut joader = Joader::new(dataset);
    let (s, r) = create_data_channel(0);
    joader.add_loader(0);
    joader.get_mut(0).unwrap().add_data_sender(s);
    let cache = Cache::new(256, &name, 1);
    tokio::spawn(async move { write(joader, cache).await });
    let mut res = tokio::spawn(async move { read_data(r).await })
        .await
        .unwrap();
    res.sort();
    assert_eq!(res, (0..len).map(|x| x as u64).collect::<Vec<_>>());
}

#[tokio::test]
async fn test_k_loader() {
    let lengths = 4096;
    let k = 8;
    let name = "dummy".to_string();
    let dataset = new_dummy(lengths, name.clone());
    let mut joader = Joader::new(dataset);
    let mut reader_map = HashMap::new();
    for id in 0..k {
        let (s, r) = create_data_channel(id as u64);
        joader.add_loader(id as u64);
        joader.get_mut(id as u64).unwrap().add_data_sender(s);
        reader_map.insert(id, tokio::spawn(async move { read_data(r).await }));
    }
    let cache = Cache::new(256, &name, 1);
    tokio::spawn(async move { write(joader, cache).await });

    for (_id, handler) in reader_map.iter_mut() {
        let mut res = handler.await.unwrap();
        res.sort();
        assert_eq!(res, (0..lengths).map(|x| x as u64).collect::<Vec<_>>());
    }
}

#[tokio::test]
async fn test_1_loader_k_sampler() {
    let lengths = 2000;
    let k = 6;
    let name = "dummy".to_string();
    let dataset = new_dummy(lengths, name.clone());
    let mut joader = Joader::new(dataset);
    let mut id_reader_map = HashMap::new();
    let mut data_reader_map = HashMap::new();
    joader.add_loader(0);
    let (s, r) = create_data_channel(0);
    joader.get_mut(0).unwrap().add_data_sender(s);
    joader.set_hash_key(k);
    data_reader_map.insert(k, tokio::spawn(async move { read_data(r).await }));
    for host_id in 0..k {
        let (s, r) = create_idx_channel(0 as u64);
        joader.get_mut(0).unwrap().add_idx_sender(s, host_id.into());
        id_reader_map.insert(host_id, tokio::spawn(async move { read_indices(r).await }));
    }
    let cache = Cache::new(256, &name, 1);
    tokio::spawn(async move { write(joader, cache).await });
    let mut res = Vec::new();
    for (id, handler) in id_reader_map.iter_mut() {
        let mut indices = handler.await.unwrap();
        println!("{} sample {} indices", id, indices.len());
        res.append(&mut indices);
    }
    for (id, handler) in data_reader_map.iter_mut() {
        let data = handler.await.unwrap();
        println!("{} read {} data", id, data.len());
        res.append(&mut data.iter().cloned().map(|x| x as u32).collect::<Vec<_>>());
    }
    res.sort();
    assert_eq!(res, (0..lengths).map(|x| x as u32).collect::<Vec<_>>());
}

#[tokio::test]
async fn test_k_loader_m_sampler() {
    let lengths = 2000;
    let k = 8;
    let m = 8;
    let name = "dummy".to_string();
    let dataset = new_dummy(lengths, name.clone());
    let mut joader = Joader::new(dataset);
    joader.set_hash_key(m);
    let mut id_reader_map = HashMap::new();
    let mut data_reader_map = HashMap::new();
    let mut res = HashMap::new();
    for loader_id in 0..k {
        res.insert(loader_id, Vec::new());
    }
    for loader_id in 0..k {
        joader.add_loader(loader_id);
        let (s, r) = create_data_channel(loader_id);
        joader.get_mut(loader_id).unwrap().add_data_sender(s);
        data_reader_map.insert(loader_id, tokio::spawn(async move { read_data(r).await }));
        for host_id in 0..m {
            let (s, r) = create_idx_channel(loader_id as u64);
            joader
                .get_mut(loader_id)
                .unwrap()
                .add_idx_sender(s, host_id.into());
            id_reader_map.insert(
                (loader_id, host_id),
                tokio::spawn(async move { read_indices(r).await }),
            );
        }
    }

    let cache = Cache::new(256, &name, 1);
    tokio::spawn(async move { write(joader, cache).await });

    for ((loader_id, host_id), handler) in id_reader_map.iter_mut() {
        let mut indices = handler.await.unwrap();
        println!("{} {}sample {} indices", host_id, loader_id, indices.len());
        res.get_mut(&loader_id).unwrap().append(&mut indices);
    }

    for (loader_id, handler) in data_reader_map.iter_mut() {
        let data = handler.await.unwrap();
        println!("{} read {} data", loader_id, data.len());
        res.get_mut(loader_id)
            .unwrap()
            .append(&mut data.iter().cloned().map(|x| x as u32).collect::<Vec<_>>());
    }

    for (_, data) in res.iter_mut() {
        data.sort();
        assert_eq!(*data, (0..lengths).map(|x| x as u32).collect::<Vec<_>>());
    }
}
