use ::joader::local_cache::cache::Cache;
use ::joader::new_joader::joader_table::JoaderTable;
use clap::{load_yaml, App};
use joader::new_service::{DatasetSvcImpl, IdGenerator, JobSvcImpl};
use joader::proto::dataset::dataset_svc_server::DatasetSvcServer;
use joader::proto::job::job_svc_server::JobSvcServer;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tonic::transport::Server;

async fn run(joader_table: Arc<Mutex<JoaderTable>>) {
    log::info!("start leader service ....");
    // sleep(Duration::from_secs_f32(0.1)).await;
    loop {
        {
            let mut joader_table = joader_table.lock().await;
            let empty = joader_table.is_empty();
            if !empty {
                joader_table.next().await;
                continue;
            }
        };
        log::debug!("sleep ....");
        sleep(Duration::from_millis(1000)).await;
        // we add it it because the mmap block, in the future, we will use io_uring
    }
}

async fn start_server(ip: &str, port: &str) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("start server");
    let id_gen = IdGenerator::new();
    let dataset_id_table = Arc::new(Mutex::new(HashMap::new()));
    let cache = Arc::new(Mutex::new(Cache::new()));
    let joader_table = Arc::new(Mutex::new(JoaderTable::new(cache)));
    let ip_port = ip.to_string() + ":" + port;
    let addr: SocketAddr = ip_port.parse()?;
    let job_id_table = Arc::new(Mutex::new(HashMap::new()));
    let dataset_svc = DatasetSvcImpl::new(
        joader_table.clone(),
        dataset_id_table.clone(),
        id_gen.clone(),
    );

    let job_svc = JobSvcImpl::new(
        joader_table.clone(),
        id_gen,
        job_id_table.clone(),
        dataset_id_table.clone(),
    );
    log::info!("start joader at {:?}......", addr);
    tokio::spawn(async move { run(joader_table) });
    let server = Server::builder()
        .add_service(DatasetSvcServer::new(dataset_svc))
        .add_service(JobSvcServer::new(job_svc))
        .serve(addr);
    server.await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let log4rs_config = matches.value_of("log4rs_config").unwrap();
    let ip = matches.value_of("ip").unwrap();
    let port = matches.value_of("port").unwrap();
    let _cache_capacity: usize = matches.value_of("cache_capacity").unwrap().parse().unwrap();
    log4rs::init_file(log4rs_config, Default::default()).unwrap();
    //start server
    start_server(ip, port).await?;
    Ok(())
}
