use crate::dataset::{Dataset, DatasetTable};
use crate::proto::common::{status::Code as RspCode, Status as RspStatus};
use crate::proto::dataset::dataset_svc_server::DatasetSvc;
use crate::proto::dataset::*;
use futures::lock::Mutex;
use std::sync::Arc;
use tonic::codegen::http::request;
use tonic::{async_trait, Request, Response, Status};
#[derive(Debug, Default)]
pub struct DatasetSvcImpl {
    dataset_table: Arc<Mutex<DatasetTable>>,
}

#[async_trait]
impl DatasetSvc for DatasetSvcImpl {
    async fn create_dataset(
        &self,
        request: Request<CreateDatasetRequest>,
    ) -> Result<Response<CreateDatasetResponse>, Status> {
        log::info!("call create dataset {:?}", request);

        // insert dataset to dataset table
        let dataset = Dataset::from_proto(request.into_inner());
        let ret = {
            let mut table = self.dataset_table.lock().await;
            table.insert(dataset)
        };
        let status = match ret {
            Err(msg) => RspStatus {
                code: RspCode::Err as i32,
                msg,
            },
            Ok(_) => RspStatus {
                code: RspCode::Ok as i32,
                msg: "succ".into(),
            },
        };
        let rsp = CreateDatasetResponse {
            status: Some(status),
        };
        Ok(Response::new(rsp))
    }
    async fn delete_dataset(
        &self,
        request: Request<DeleteDatasetRequest>,
    ) -> Result<Response<DeleteDatasetResponse>, Status> {
        log::info!("call delete dataset {:?}", request);

        let ret = {
            let mut table = self.dataset_table.lock().await;
            table.remove(&request.into_inner().name)
        };

        let status = match ret {
            Err(msg) => RspStatus {
                code: RspCode::Err as i32,
                msg,
            },
            Ok(_) => RspStatus {
                code: RspCode::Ok as i32,
                msg: "succ".into(),
            },
        };
        let rsp = DeleteDatasetResponse {
            status: Some(status),
        };
        Ok(Response::new(rsp))
    }
}
