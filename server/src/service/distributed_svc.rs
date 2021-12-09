use crate::joader::joader_table::JoaderTable;
use crate::loader::{create_idx_channel, IdxReceiver};
use crate::proto::distributed::distributed_svc_server::DistributedSvc;
use crate::proto::distributed::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{async_trait, Request, Response, Status};

use super::GlobalID;

#[derive(Debug, Default)]
struct Host {
    recv: HashMap<u64, IdxReceiver>,
}

impl Host {
    fn add(&mut self, r: IdxReceiver) {
        self.recv.insert(r.get_loader_id(), r);
    }

    fn del(&mut self, loader_id: u64) {
        self.recv.remove(&loader_id);
    }

    async fn recv_all(&mut self) -> Vec<SampleResult> {
        let mut del_loaders = Vec::new();
        let mut ret = Vec::new();
        for (loader_id, v) in self.recv.iter_mut() {
            let (indices, empty) = v.recv_all().await;
            if empty {
                del_loaders.push(*loader_id);
            }
            ret.push(SampleResult {
                loader_id: *loader_id,
                indices,
            });
        }
        for id in del_loaders {
            self.recv.remove(&id);
        }
        ret
    }
}

#[derive(Debug)]
pub struct DistributedSvcImpl {
    loader_id: GlobalID,
    host_id: GlobalID,
    loader_id_table: Arc<Mutex<HashMap<String, u64>>>,
    host_id_table: Arc<Mutex<HashMap<String, u64>>>,
    host_table: Arc<Mutex<HashMap<u64, Host>>>,
    host_port_table: Arc<Mutex<HashMap<String, u64>>>,
    joader_table: Arc<Mutex<JoaderTable>>,
}

#[async_trait]
impl DistributedSvc for DistributedSvcImpl {
    async fn register_host(
        &self,
        request: Request<RegisterHostRequest>,
    ) -> Result<Response<RegisterHostResponse>, Status> {
        let request = request.into_inner();
        let mut table = self.host_id_table.lock().await;
        if table.contains_key(&request.ip) {
            return Err(Status::already_exists(format!("{}", request.ip)));
        }
        let id = self.host_id.get_id().await;
        // 1. host id table: ip -> id
        table.insert(request.ip.clone(), id);
        // 2. host port table: ip -> port
        self.host_port_table
            .lock()
            .await
            .insert(request.ip, request.port);
        // 3. host table: id -> host
        self.host_table.lock().await.insert(id, Host::default());

        // 4. update host number
        let mut jt = self.joader_table.lock().await;
        jt.set_hash_key(table.len() as u32);
        Ok(Response::new(RegisterHostResponse { host_id: id }))
    }

    async fn delete_host(
        &self,
        _request: Request<DeleteHostRequest>,
    ) -> Result<Response<DeleteHostResponse>, Status> {
        Err(Status::unimplemented(
            "Delete host has not been implemented",
        ))
    }

    async fn create_sampler(
        &self,
        request: Request<CreateSamplerRequest>,
    ) -> Result<Response<CreateSamplerResponse>, Status> {
        let request = request.into_inner();
        let host_id = *self
            .host_id_table
            .lock()
            .await
            .get(&request.ip)
            .ok_or_else(|| Status::not_found(format!("{} not exited", request.ip)))?;

        let mut loader_id_table = self.loader_id_table.lock().await;
        let mut jt = self.joader_table.lock().await;

        let joader = jt
            .get_mut(&request.dataset_name)
            .map_err(|x| Status::not_found(x))?;
        // 1. If loader not exited, add loader and update loader_id
        let loader_id;
        if loader_id_table.contains_key(&request.name) {
            loader_id = loader_id_table[&request.name];
        } else {
            loader_id = self.loader_id.get_id().await;
            joader.add_loader(loader_id);
            loader_id_table.insert(request.name.clone(), loader_id);
        }

        // 2. Add sample to loader
        let loader = joader
            .get_mut(loader_id)
            .map_err(|x| Status::not_found(x))?;
        let (is, ir) = create_idx_channel(loader_id);
        loader.add_idx_sender(is, host_id);
        // 2. Add recv to host
        let mut ht = self.host_table.lock().await;
        let host = ht.get_mut(&host_id).unwrap();
        host.add(ir);

        let length = joader.len();
        Ok(Response::new(CreateSamplerResponse { length, loader_id }))
    }

    async fn delete_sampler(
        &self,
        request: Request<DeleteSamplerRequest>,
    ) -> Result<Response<DeleteSamplerResponse>, Status> {
        let request = request.into_inner();
        let host_id = *self
            .host_id_table
            .lock()
            .await
            .get(&request.ip)
            .ok_or_else(|| Status::not_found(format!("{} not exited", request.ip)))?;
        let mut ht = self.host_table.lock().await;

        let mut loader_id_table = self.loader_id_table.lock().await;
        let mut jt = self.joader_table.lock().await;
        //1. loader remove host
        let joader = jt
            .get_mut(&request.dataset_name)
            .map_err(|x| Status::not_found(x))?;
        let loader_id = loader_id_table
            .get(&request.name)
            .ok_or_else(|| Status::not_found(format!("{} not exited", request.name)))?;
        let loader = joader.get_mut(*loader_id).unwrap();
        loader.del_idx_sender(host_id);
        //2. host remove recv
        ht.get_mut(&host_id).unwrap().del(*loader_id);
        //3. if empty, remove host_id
        if loader.is_empty() {
            loader_id_table.remove(&request.name);
        }
        Ok(Response::new(DeleteSamplerResponse {}))
    }

    async fn query_host(
        &self,
        request: Request<QueryHostRequest>,
    ) -> Result<Response<QueryHostResponse>, Status> {
        let req = request.into_inner();
        let port = *self
            .host_port_table
            .lock()
            .await
            .get(&req.ip)
            .ok_or_else(|| Status::not_found(format!("Host {} not exist", req.ip)))?;
        Ok(Response::new(QueryHostResponse { port }))
    }

    async fn sample(
        &self,
        request: Request<SampleRequest>,
    ) -> Result<Response<SampleResponse>, Status> {
        let req = request.into_inner();
        let host_id = self.host_id_table.lock().await[&req.ip];
        let mut ht = self.host_table.lock().await;

        Ok(Response::new(SampleResponse {
            res: ht.get_mut(&host_id).unwrap().recv_all().await,
        }))
    }
}
