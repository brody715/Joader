use crate::joader::joader_table::JoaderTable;
use crate::proto::dataset::dataset_svc_server::DatasetSvc;
use crate::proto::dataset::*;
use crate::{dataset, joader::joader::Joader};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{async_trait, Request, Response, Status};

use super::to_status;
#[derive(Debug)]
pub struct DatasetSvcImpl {
    joader_table: Arc<Mutex<JoaderTable>>,
}


impl DatasetSvcImpl {
    pub fn new(joader_table: Arc<Mutex<JoaderTable>>) -> DatasetSvcImpl {
        DatasetSvcImpl {
            joader_table,
        }
    }
}


#[async_trait]
impl DatasetSvc for DatasetSvcImpl {
    async fn create_dataset(
        &self,
        request: Request<CreateDatasetRequest>,
    ) -> Result<Response<CreateDatasetResponse>, Status> {
        log::info!("call create dataset {:?}", request);
        // insert dataset to dataset table
        let joader = Joader::new(dataset::from_proto(request.into_inner()));
        let ret = self.joader_table.lock().await.add_joader(joader);
        Ok(Response::new(CreateDatasetResponse {
            status: Some(to_status(&ret)),
        }))
    }

    async fn delete_dataset(
        &self,
        request: Request<DeleteDatasetRequest>,
    ) -> Result<Response<DeleteDatasetResponse>, Status> {
        log::info!("call delete dataset {:?}", request);

        let ret = {
            let mut table = self.joader_table.lock().await;
            table.del_joader(&request.into_inner().name)
        };
        Ok(Response::new(DeleteDatasetResponse {
            status: Some(to_status(&ret)),
        }))
    }
}
