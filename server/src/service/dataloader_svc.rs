use crate::joader::joader_table::JoaderTable;
use crate::loader::{create_data_channel, DataReceiver};
use crate::proto::dataloader::data_loader_svc_server::DataLoaderSvc;
use crate::proto::dataloader::*;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{async_trait, Request, Response, Status};

use super::{GlobalID, IDTable};

#[derive(Debug)]
pub struct DataLoaderSvcImpl {
    joader_table: Arc<Mutex<JoaderTable>>,
    loader_id_table: IDTable,
    delete_loaders: Arc<Mutex<HashSet<u64>>>,
    recv_table: Arc<Mutex<HashMap<u64, DataReceiver>>>,
    loader_id: GlobalID,
}

impl DataLoaderSvcImpl {
    pub fn new(
        joader_table: Arc<Mutex<JoaderTable>>,
        delete_loaders: Arc<Mutex<HashSet<u64>>>,
        loader_id: GlobalID,
        loader_id_table: IDTable,
    ) -> Self {
        Self {
            joader_table,
            recv_table: Default::default(),
            delete_loaders,
            loader_id_table,
            loader_id,
        }
    }
}

#[async_trait]
impl DataLoaderSvc for DataLoaderSvcImpl {
    async fn create_dataloader(
        &self,
        request: Request<CreateDataloaderRequest>,
    ) -> Result<Response<CreateDataloaderResponse>, Status> {
        log::info!("call create loader {:?}", request);
        let request = request.into_inner();
        let mut loader_id_table = self.loader_id_table.lock().await;
        let mut jt = self.joader_table.lock().await;
        let mut rt = self.recv_table.lock().await;
        let joader = jt
            .get_mut(&request.dataset_name)
            .map_err(|x| Status::not_found(x))?;
        // 1. Update loader id table
        let loader_id;
        if loader_id_table.contains_key(&request.name) {
            loader_id = loader_id_table[&request.name];
        } else {
            loader_id = self.loader_id.get_id().await;
            loader_id_table.insert(request.name.clone(), loader_id);
            // 2. If not exited, add loader
            joader.add_loader(loader_id);
        }
        // 3 update recv_table
        let loader = joader
            .get_mut(loader_id)
            .map_err(|x| Status::not_found(x))?;
        let (ds, dr) = create_data_channel(loader_id);
        loader.add_data_sender(ds);
        if rt.contains_key(&loader_id) {
            return Err(Status::already_exists(format!(
                "Loader {} has exited",
                loader_id
            )));
        }
        rt.insert(loader_id, dr);

        Ok(Response::new(CreateDataloaderResponse {
            length: joader.len(),
            shm_path: jt.get_shm_path(),
            loader_id,
            status: None,
        }))
    }
    async fn next(&self, request: Request<NextRequest>) -> Result<Response<NextResponse>, Status> {
        let loader_id = request.into_inner().loader_id;
        let mut delete_loaders = self.delete_loaders.lock().await;
        if delete_loaders.contains(&loader_id) {
            return Err(Status::out_of_range(format!("data has used up")));
        }
        let mut loader_table = self.recv_table.lock().await;
        let recv = loader_table
            .get_mut(&loader_id)
            .ok_or_else(|| Status::not_found(format!("Loader {} not found", loader_id)))?;
        let (address, empty) = recv.recv_all().await;
        if empty {
            delete_loaders.insert(loader_id);
        }
        Ok(Response::new(NextResponse { address }))
    }
    async fn delete_dataloader(
        &self,
        request: Request<DeleteDataloaderRequest>,
    ) -> Result<Response<DeleteDataloaderResponse>, Status> {
        log::info!("call delete loader {:?}", request);
        let request = request.into_inner();
        let mut id_table = self.loader_id_table.lock().await;
        let loader_id = id_table[&request.name];

        let mut rt = self.recv_table.lock().await;
        // 1 remove recv table
        rt.remove(&loader_id);

        // 2 remove loader
        let mut jt = self.joader_table.lock().await;
        let joader = jt
            .get_mut(&request.dataset_name)
            .map_err(|x| Status::not_found(x))?;
        let loader = joader
            .get_mut(loader_id)
            .map_err(|x| Status::not_found(x))?;

        loader.del_data_sender();

        // 3 if all subhost have removed in loader, then remove loader_id
        if loader.is_empty() {
            id_table.remove(&request.name);
        }
        Ok(Response::new(DeleteDataloaderResponse {}))
    }
}
