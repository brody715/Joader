use super::to_status;
use crate::joader::joader_table::JoaderTable;
use crate::loader;
use crate::loader::Rloader;
use crate::proto::dataloader::data_loader_svc_server::DataLoaderSvc;
use crate::proto::dataloader::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{async_trait, Request, Response, Status};

#[derive(Debug, Default)]
pub struct DataLoaderSvcImpl {
    joader_table: Arc<Mutex<JoaderTable>>,
    loader_table: Arc<Mutex<HashMap<u64, Rloader>>>,
    id: Arc<Mutex<u64>>,
}

#[async_trait]
impl DataLoaderSvc for DataLoaderSvcImpl {
    async fn create_dataloader(
        &self,
        request: Request<CreateDataloaderRequest>,
    ) -> Result<Response<CreateDataloaderResponse>, Status> {
        log::info!("call create loader {:?}", request);
        let mut id = self.id.lock().await;
        *id += 1;
        let loader_id: u64 = *id;
        let (s, r) = loader::from_proto(request.into_inner());
        let mut loader_table = self.loader_table.lock().await;
        loader_table.insert(loader_id, r);

        let shm_path;
        let ret = {
            let mut joader_table = self.joader_table.lock().await;
            shm_path = joader_table.get_shm_path();
            joader_table.add_loader(s)
        };

        Ok(Response::new(CreateDataloaderResponse {
            shm_path,
            loader_id,
            status: Some(to_status(&ret)),
        }))
    }
    async fn next(&self, request: Request<NextRequest>) -> Result<Response<NextResponse>, Status> {
        log::info!("call next {:?}", request);
        let loader_id = request.into_inner().loader_id;

        let address = {
            let mut loader_table = self.loader_table.lock().await;
            loader_table.get_mut(&loader_id).unwrap().next()
        };
        Ok(Response::new(NextResponse { address }))
    }
    async fn delete_dataloader(
        &self,
        request: Request<DeleteDataloaderRequest>,
    ) -> Result<Response<DeleteDataloaderResponse>, Status> {
        log::info!("call delete loader {:?}", request);

        let mut status = None;
        let mut loader_table = self.loader_table.lock().await;
        if let Some(loader) = loader_table.remove(&request.into_inner().loader_id) {
            let mut joader_table = self.joader_table.lock().await;
            status = Some(to_status(&joader_table.del_loader(loader)));
        }
        Ok(Response::new(DeleteDataloaderResponse { status }))
    }
}
