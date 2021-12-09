use super::Dataset;
use super::DatasetRef;
use crate::{
    cache::cache::Cache,
    proto::dataset::{CreateDatasetRequest, DataItem},
};
use std::{fmt::Debug, sync::Arc};

#[derive(Clone, Default, Debug)]
pub struct FileDataset {
    items: Vec<DataItem>,
    _root: String,
    name: String,
}

pub fn from_proto(request: CreateDatasetRequest) -> DatasetRef {
    let name = request.name;
    let items = request.items;
    Arc::new(FileDataset {
        items,
        _root: "".into(),
        name,
    })
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

    fn read(&self, _cache: &mut Cache, idx: u32, _ref_cnt: usize) -> u64 {
        idx as u64
    }

    fn len(&self) -> u64 {
        self.items.len() as u64
    }
}
