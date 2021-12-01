use crate::{
    cache::cache::Cache,
    proto::dataset::{CreateDatasetRequest, DataItem},
};
use std::sync::Arc;

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
pub trait Dataset {
    fn get_name(&self) -> &str;
    fn get_indices(&self) -> Vec<u64>;
    fn read(&self, cache: &mut Cache) -> u64;
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

    fn get_indices(&self) -> Vec<u64> {
        let start = 0u64;
        let end = self.items.len() as u64;
        (start..end).collect::<Vec<_>>()
    }

    fn read(&self, cache: &mut Cache) -> u64 {
        todo!()
    }
}
