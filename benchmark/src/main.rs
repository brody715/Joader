use ::joader::cache::cache::Cache;
use ::joader::joader::joader_table::JoaderTable;
use joader::proto::dataloader::data_loader_svc_server::DataLoaderSvcServer;
use joader::proto::dataset::dataset_svc_server::DatasetSvcServer;
use joader::service::{DataLoaderSvcImpl, DatasetSvcImpl};
use libc::shm_unlink;
use std::net::SocketAddr;
use std::process;
use std::sync::Arc;
use std::{thread, time::Duration};
use tokio::sync::Mutex;
use tonic::transport::Server;

// const log4rs_config: &str = "log4rs.yaml";
const HOST: &str = "127.0.0.1:4321";
const SHM_PATH: &str = "DLCache";
const HEAD_NUM: u64 = 2048;
const CACHE_CAPACITY: usize = 1024*1024*1024;


async fn start(joader_table: Arc<Mutex<JoaderTable>>) {
    println!("start joader loop ....");
    loop {
        let mut joader_table = joader_table.lock().await;
        if joader_table.is_empty() {
            thread::sleep(Duration::from_millis(1000));
            continue;
        }
        coz::progress!();
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //start joader_table
    todo!()
}
