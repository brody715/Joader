use clap::Parser;
use joader::proto::dataset::dataset_svc_server::{DatasetSvc, DatasetSvcServer};
use joader::service::DatasetSvcImpl;
use std::net::SocketAddr;
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    log4rs::init_file(opts.log4rs_config, Default::default()).unwrap();
    let addr: SocketAddr = opts.host.parse()?;
    let dataset_svc = DatasetSvcImpl::default();
    log::info!("start joader at {:?}......\n", addr);
    Server::builder()
        .add_service(DatasetSvcServer::new(dataset_svc))
        .serve(addr)
        .await?;
    Ok(())
}
