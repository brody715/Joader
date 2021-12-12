use crate::joader::joader_table::JoaderTable;
use crate::proto::dataset::dataset_svc_server::DatasetSvc;
use crate::proto::dataset::*;
use crate::{dataset, joader::joader::Joader};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{async_trait, Request, Response, Status};

use super::GlobalID;
#[derive(Debug)]
pub struct DatasetSvcImpl {
    joader_table: Arc<Mutex<JoaderTable>>,
    dataset_table: Arc<Mutex<HashMap<String, u32>>>,
    id: GlobalID,
}

impl DatasetSvcImpl {
    pub fn new(
        joader_table: Arc<Mutex<JoaderTable>>,
        dataset_table: Arc<Mutex<HashMap<String, u32>>>,
        id: GlobalID,
    ) -> DatasetSvcImpl {
        Self {
            joader_table,
            dataset_table,
            id,
        }
    }
}

#[async_trait]
impl DatasetSvc for DatasetSvcImpl {
    async fn create_dataset(
        &self,
        request: Request<CreateDatasetRequest>,
    ) -> Result<Response<CreateDatasetResponse>, Status> {
        let request = request.into_inner();
        let dt = self.dataset_table.lock().await;
        if dt.contains_key(&request.name) {
            return Err(Status::already_exists(format!(
                "{:?} has already existed",
                request
            )));
        }

        log::debug!("call create dataset {:?}", request);
        let id = self.id.get_dataset_id().await;
        // insert dataset to dataset table
        let joader = Joader::new(dataset::build_dataset(request, id));
        self.joader_table.lock().await.add_joader(joader);
        Ok(Response::new(CreateDatasetResponse { status: None }))
    }

    async fn delete_dataset(
        &self,
        request: Request<DeleteDatasetRequest>,
    ) -> Result<Response<DeleteDatasetResponse>, Status> {
        log::debug!("call delete dataset {:?}", request);
        let request = request.into_inner();
        let dt = self.dataset_table.lock().await;
        let mut jt = self.joader_table.lock().await;
        match dt.get(&request.name) {
            Some(id) => {
                jt.del_joader(*id);
                Ok(Response::new(DeleteDatasetResponse { status: None }))
            }
            None => Err(Status::not_found(format!(
                "{:?} has already not found",
                request
            ))),
        }
    }
}
