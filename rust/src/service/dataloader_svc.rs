use crate::loader::{self, Loader, LoaderTable};
use crate::proto::dataloader::data_loader_svc_server::DataLoaderSvc;
use crate::proto::dataloader::*;
use futures::lock::Mutex;
use std::sync::Arc;
use tonic::codegen::http::request;
use tonic::{async_trait, Request, Response, Status};

use super::to_status;
#[derive(Debug, Default)]
pub struct DataLoaderSvcImpl {
    loader_table: Arc<Mutex<LoaderTable>>,
}

#[async_trait]
impl DataLoaderSvc for DataLoaderSvcImpl {
    async fn create_dataloader(
        &self,
        request: Request<CreateDataloaderRequest>,
    ) -> Result<Response<CreateDataloaderResponse>, Status> {
        log::info!("call create loader {:?}", request);
        let loader = Loader::from_proto(request.into_inner());
        let ret = {
            let mut loader_table = self.loader_table.lock().await;
            loader_table.insert(loader)
        };
        Ok(Response::new(CreateDataloaderResponse {
            shared_mem_file: "".into(),
            loader_id: 1,
            status: Some(to_status(ret)),
        }))
    }
    async fn next(&self, request: Request<NextRequest>) -> Result<Response<NextResponse>, Status> {
        todo!()
    }
    async fn delete_dataloader(
        &self,
        request: Request<DeleteDataloaderRequest>,
    ) -> Result<Response<DeleteDataloaderResponse>, Status> {
        log::info!("call delete loader {:?}", request);
    }
}
