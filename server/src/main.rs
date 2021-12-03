use ::joader::cache::cache::Cache;
use ::joader::joader::joader_table::JoaderTable;
use clap::Parser;
use joader::proto::dataloader::data_loader_svc_server::DataLoaderSvcServer;
use joader::proto::dataset::dataset_svc_server::DatasetSvcServer;
use joader::service::{DataLoaderSvcImpl, DatasetSvcImpl};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tonic::transport::Server;

#[derive(Parser)]
struct Opts {
    // The custom log4rs config file.
    #[clap(long, default_value = "log4rs.yaml")]
    log4rs_config: String,
    #[clap(long, default_value = "127.0.0.1:4321")]
    host: String,
    #[clap(long, default_value = "DLCache")]
    shm_path: String,
    #[clap(long, default_value = "128")]
    head_num: u64,
    #[clap(long, default_value = "1048576")]
    cache_capacity: usize,
}

async fn start(joader_table: Arc<Mutex<JoaderTable>>) {
    log::info!("start joader loop ....");
    loop {
        let mut joader_table = joader_table.lock().await;
        if joader_table.is_empty() {
            log::info!("sleep ....");
            sleep(Duration::from_millis(500)).await;
            continue;
        }
        joader_table.next().await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    log4rs::init_file(opts.log4rs_config, Default::default()).unwrap();
    //start joader_table
    let cache = Cache::new(opts.cache_capacity, opts.shm_path, opts.head_num);
    let joader_table = Arc::new(Mutex::new(JoaderTable::new(cache)));
    // start server
    let addr: SocketAddr = opts.host.parse()?;
    let dataset_svc = DatasetSvcImpl::new(joader_table.clone());
    let data_loader_svc = DataLoaderSvcImpl::new(joader_table.clone());

    // start joader
    tokio::spawn(async move { start(joader_table).await });

    log::info!("start joader at {:?}......\n", addr);
    Server::builder()
        .add_service(DatasetSvcServer::new(dataset_svc))
        .add_service(DataLoaderSvcServer::new(data_loader_svc))
        .serve(addr)
        .await?;
    Ok(())
}
