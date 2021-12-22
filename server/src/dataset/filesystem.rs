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
    id: u32,
}

pub fn from_proto(request: CreateDatasetRequest, id: u32) -> DatasetRef {
    let items = request.items;
    Arc::new(FileDataset {
        items,
        _root: "".into(),
        id,
    })
}

impl Dataset for FileDataset {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_indices(&self) -> Vec<u32> {
        let start = 0u32;
        let end = self.items.len() as u32;
        (start..end).collect::<Vec<_>>()
    }

    fn read(&self, _cache: &mut Cache, idx: u32, _ref_cnt: usize, _loader_cnt: usize) -> u64 {
        idx as u64
    }

    fn len(&self) -> u64 {
        self.items.len() as u64
    }
}
