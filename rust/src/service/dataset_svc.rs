use crate::proto::common::status;
use crate::proto::common::{status::Code as RspCode, Status as RspStatus};
use crate::proto::dataset::dataset_svc_server::DatasetSvc;
use crate::proto::dataset::*;
use tonic::codegen::http::request;
use tonic::{async_trait, Request, Response, Status};
#[derive(Debug, Default)]
pub struct DatasetSvcImpl {}

#[async_trait]
impl DatasetSvc for DatasetSvcImpl {
    async fn create_dataset(
        &self,
        request: Request<CreateDatasetRequest>,
    ) -> Result<Response<CreateDatasetResponse>, Status> {
        log::info!("call create dataset {:?}", request);
        let status = RspStatus {
            code: RspCode::Ok as i32,
            msg: "succ".into(),
        };
        let rsp = CreateDatasetResponse { status: Some(status) };
        Ok(Response::new(rsp))
    }
    async fn delete_dataset(
        &self,
        request: Request<DeleteDatasetRequest>,
    ) -> Result<Response<DeleteDatasetResponse>, Status> {
        log::info!("call delete dataset {:?}", request);
        let status = RspStatus {
            code: RspCode::Ok as i32,
            msg: "succ".into(),
        };
        let rsp = DeleteDatasetResponse { status: Some(status) };
        Ok(Response::new(rsp))
    }
}
