use crate::dataset::{Dataset, DatasetTable};
use crate::proto::dataset::dataset_svc_server::DatasetSvc;
use crate::proto::dataset::*;
use futures::lock::Mutex;
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};

use super::to_status;
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
        Ok(Response::new(CreateDatasetResponse {
            status: Some(to_status(ret)),
        }))
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
        Ok(Response::new(DeleteDatasetResponse {
            status: Some(to_status(ret)),
        }))
    }
}
