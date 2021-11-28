use std::net::SocketAddr;

use joader::proto::dataset::dataset_svc_server::{DatasetSvc, DatasetSvcServer};
use joader::service::DatasetSvcImpl;
use tonic::transport::Server;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let addr: SocketAddr = "127.0.0.1:4321".parse()?;
    let dataset_svc = DatasetSvcImpl::default();
    log::info!("start joader at {:?}......\n", addr);
    Server::builder()
        .add_service(DatasetSvcServer::new(dataset_svc))
        .serve(addr)
        .await?;
    Ok(())
}
