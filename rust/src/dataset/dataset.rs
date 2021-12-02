use crate::{
    cache::cache::Cache,
    proto::dataset::{CreateDatasetRequest, DataItem},
};
use std::{fmt::Debug, sync::Arc};

pub type DatasetRef = Arc<dyn Dataset>;

pub fn from_proto(request: CreateDatasetRequest) -> DatasetRef {
    let name = request.name;
    let items = request.items;
    Arc::new(FileDataset {
        items,
        root: "".into(),
        name,
    })
}
pub trait Dataset: Sync + Send + Debug {
    fn get_name(&self) -> &str;
    fn get_indices(&self) -> Vec<u32>;
    fn read(&self, cache: &mut Cache, idx: u32) -> u64;
}

#[derive(Clone, Default, Debug)]
pub struct FileDataset {
    items: Vec<DataItem>,
    root: String,
    name: String,
}

impl Dataset for FileDataset {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_indices(&self) -> Vec<u32> {
        let start = 0u32;
        let end = self.items.len() as u32;
        (start..end).collect::<Vec<_>>()
    }

    fn read(&self, _cache: &mut Cache, idx: u32) -> u64 {
        idx as u64
    }
}
