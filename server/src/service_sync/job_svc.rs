use super::{decode_addr_read_off, IDGenerator, IDTable};
use crate::joader::joader_table::JoaderTable;
use crate::loader::{create_data_channel, DataReceiver};
use crate::proto::dataloader::data_loader_svc_server::DataLoaderSvc;
use crate::proto::dataloader::*;
use crate::proto::distributed::distributed_svc_client::DistributedSvcClient;
use crate::proto::distributed::{CreateSamplerRequest, DeleteSamplerRequest};
use crate::Role;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{async_trait, Request, Response, Status};

#[derive(Debug)]
pub struct DataLoaderSvcImpl {
    id_gen: IDGenerator,
    joader_table: Arc<Mutex<JoaderTable>>,
    job_id_table: IDTable,
    recv_table: Arc<Mutex<HashMap<u64, DataReceiver>>>,
    ds_id_table: Arc<Mutex<HashMap<String, u32>>>,
}

impl DataLoaderSvcImpl {
    pub fn new(
        joader_table: Arc<Mutex<JoaderTable>>,
        job_id_table: IDTable,
        ds_id_table: Arc<Mutex<HashMap<String, u32>>>,
        id_gen: IDGenerator,
    ) -> Self {
        Self {
            joader_table,
            recv_table: Default::default(),
            // delete_loaders,
            job_id_table,
            id_gen,
            ds_id_table,
        }
    }
}

#[async_trait]
impl DataLoaderSvc for DataLoaderSvcImpl {
    async fn create_job(
        &self,
        request: Request<CreateDataloaderRequest>,
    ) -> Result<Response<CreateDataloaderResponse>, Status> {
        log::info!("call create loader {:?}", request);
        let request = request.into_inner();
        let mut rt = self.recv_table.lock().await;
        let mut jt = self.joader_table.lock().await;
        let mut job_id_table = self.job_id_table.lock().await;
        let dt = self.ds_id_table.lock().await;
        let length;
        let loader_id;
        let dataset_id = *dt
            .get(&request.dataset_name)
            .ok_or_else(|| Status::not_found(&request.dataset_name))?;
        
        let joader = jt.get_mut(dataset_id);
        let job_id = self.id.get_job_id(dataset_id).await;
        job_id_table.insert(request.name.clone(), loader_id);
        joader.add_loader(loader_id, request.nums);
        let length = joader.len();
        // 3 update recv_table
        let (ds, dr) = create_data_channel(loader_id);
        joader.add_data_sender(loader_id, ds);
        rt.insert(loader_id, dr);
        Ok(Response::new(CreateDataloaderResponse {
            length,
            shm_path: jt.get_shm_path(),
            loader_id,
            status: None,
        }))
    }

    async fn next(&self, request: Request<NextRequest>) -> Result<Response<NextResponse>, Status> {
        let request = request.into_inner();
        let loader_id = request.loader_id;
        let bs = request.batch_size;
        let mut delete_loaders = self.delete_loaders.lock().await;
        let mut rt = self.recv_table.lock().await;
        if delete_loaders.contains(&loader_id) {
            return Err(Status::out_of_range(format!("data has used up")));
        }
        
        let recv = rt
            .get_mut(&loader_id)
            .ok_or_else(|| Status::not_found(format!("Loader {} not found", loader_id)))?;
        let (recv_data, empty) = match bs {
            -1 => recv.recv_all().await,
            _ => recv.recv_batch(bs as u32).await,
        };
        if empty {
            delete_loaders.insert(loader_id);
        }
        let mut address = Vec::with_capacity(recv_data.len());
        let mut read_off = Vec::with_capacity(recv_data.len());
        for data in recv_data {
            let (a, r) = decode_addr_read_off(data);
            address.push(a);
            read_off.push(r);
        }
        
        Ok(Response::new(NextResponse { address, read_off }))
    }

    async fn delete_dataloader(
        &self,
        request: Request<DeleteDataloaderRequest>,
    ) -> Result<Response<DeleteDataloaderResponse>, Status> {
        log::info!("call delete loader {:?}", request);
        let request = request.into_inner();
        let mut rt = self.recv_table.lock().await;
        let mut jt = self.joader_table.lock().await;
        let mut id_table = self.loader_id_table.lock().await;
        let loader_id = id_table[&request.name];
        println!("lock success");
        // 1 remove loader
        let dataset_id = GlobalID::parse_dataset_id(loader_id);
        let joader = jt.get_mut(dataset_id);
        joader.del_data_sender(loader_id);
        // 2 remove recv table
        rt.remove(&loader_id);
        // 3 if all subhost have removed in loader, then remove loader_id
        if joader.is_loader_empty(loader_id) {
            id_table.remove(&request.name);
            joader.del_loader(loader_id);
        }

        if let Some(mut leader) = self.leader.clone() {
            leader
                .delete_sampler(DeleteSamplerRequest {
                    name: request.name,
                    dataset_name: request.dataset_name,
                    ip: self.ip.to_string(),
                })
                .await
                .unwrap();
        }
        
        Ok(Response::new(DeleteDataloaderResponse {}))
    }

    async fn reset_dataloader(
        &self,
        request: Request<ResetDataloaderRequest>,
    ) -> Result<Response<ResetDataloaderResponse>, Status> {
        log::info!("call reset loader {:?}", request);
        let request = request.into_inner();
        let mut jt = self.joader_table.lock().await;
        let id_table = self.loader_id_table.lock().await;
        let loader_id = id_table[&request.name];
        let dataset_id = GlobalID::parse_dataset_id(loader_id);
        let joader = jt.get_mut(dataset_id);
        joader.reset_dataloader(loader_id);
        Ok(Response::new(ResetDataloaderResponse {}))
    }
}
