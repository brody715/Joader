use std::sync::Arc;
use std::time::{SystemTime, Duration};
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;
use tokio::time::sleep;

use super::joader::*;
use crate::dataset::build_dataset;
use crate::proto::dataset::{CreateDatasetRequest, DataItem};
use crate::proto::job::Data;
use crate::{
    job::Job, cache::cache::Cache, dataset::new_dummy,
    joader::joader_table::JoaderTable,
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

async fn read(_job_id: u64, mut recv: Receiver<Arc<Vec<Data>>>, len: usize, dur: Duration) -> Vec<Arc<Vec<Data>>> {
    let now = SystemTime::now();
    let mut res = Vec::new();
    loop {
        let data = recv.recv().await;
        sleep(dur).await;
        println!("read {}", res.len());
        match data {
            Some(data) => res.push(data),
            None => continue,
        }
        if res.len() == len {
            break;
        }
    }
    let time = SystemTime::now().duration_since(now).unwrap().as_secs_f32();
    println!("get each data cost {:} secs", time / len as f32);
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
    tokio::spawn(async move { read(0, recv, len, Duration::from_millis(1)).await })
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_joader_lmdb() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let cache = Arc::new(Mutex::new(Cache::new()));
    let mut jt = JoaderTable::new(cache);

    let len = 4096;
    let location = "/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb".to_string();
    let name = "lmdb".to_string();
    let items = (0..len)
        .map(|x| DataItem {
            keys: vec![x.to_string()],
        })
        .collect::<Vec<_>>();
    let proto = CreateDatasetRequest {
        name,
        location,
        r#type: crate::proto::dataset::create_dataset_request::Type::Lmdb as i32,
        items,
        weights: vec![0],
    };
    let dataset = build_dataset(proto, 0);
    let mut joader = Joader::new(dataset);
    let (job, recv) = Job::new(0);
    joader.add_job(job.clone()).await;
    jt.add_joader(joader);
    tokio::spawn(async move { write(jt, len).await });
    tokio::spawn(async move { read(0, recv, len, Duration::from_millis(1)).await })
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 32)]
async fn test_joader_multi_lmdb() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let cache = Arc::new(Mutex::new(Cache::new()));
    let mut jt = JoaderTable::new(cache);

    let len = 2048;
    let location = "/home/xiej/data/lmdb-imagenet/ILSVRC-train.lmdb".to_string();
    let name = "lmdb".to_string();
    let items = (0..len)
        .map(|x| DataItem {
            keys: vec![x.to_string()],
        })
        .collect::<Vec<_>>();
    let proto = CreateDatasetRequest {
        name,
        location,
        r#type: crate::proto::dataset::create_dataset_request::Type::Lmdb as i32,
        items,
        weights: vec![0],
    };
    let dataset = build_dataset(proto, 0);
    let mut joader = Joader::new(dataset);
    let mut reader = Vec::new();
    for i in 0..5 {
        let (job, recv) = Job::new(i);
        joader.add_job(job.clone()).await;
        reader.push(tokio::spawn(async move { read(i, recv, len, Duration::from_millis(i)).await }));
    }
    jt.add_joader(joader);
    tokio::spawn(async move { write(jt, len).await }).await.unwrap();
    for r in reader {
        r.await.unwrap();
    }

}
