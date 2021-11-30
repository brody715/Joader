use std::sync::Arc;

use crate::proto::dataset::{CreateDatasetRequest, DataItem};
use crossbeam::channel::Sender;

pub type DatasetRef = Arc<dyn Dataset>;

#[derive(Clone, Debug)]
pub enum DatasetType {
    FileSystem(String),
    LMDB(String),
    None,
}

impl Default for DatasetType {
    fn default() -> DatasetType {
        DatasetType::None
    }
}
pub struct DataRequest {
    pub sender: Vec<Sender<u64>>,
    pub key: DataItem,
    pub dataset: DatasetType,
}

pub fn from_proto(request: CreateDatasetRequest) -> DatasetRef {
    todo!()
}
pub trait Dataset {}

#[derive(Clone, Default, Debug)]
pub struct FileDataset {
    items: Vec<DataItem>,
}
