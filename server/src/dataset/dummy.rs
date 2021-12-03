use crate::{
    cache::cache::Cache,
    proto::dataset::{CreateDatasetRequest, DataItem},
};
use std::{fmt::Debug, sync::Arc};
use super::Dataset;
use super::DatasetRef;

#[derive(Clone, Default, Debug)]
struct DummyDataset {
    magic: u8,
    items: Vec<DataItem>,
    name: String,
}

pub fn from_proto(request: CreateDatasetRequest) -> DatasetRef {
    let name = request.name;
    let items = request.items;
    Arc::new(DummyDataset {
        items,
        magic: 7u8,
        name,
    })
}

impl Dataset for DummyDataset {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_indices(&self) -> Vec<u32> {
        let start = 0u32;
        let end = self.items.len() as u32;
        (start..end).collect::<Vec<_>>()
    }
    fn read(&self, cache: &mut Cache, idx: u32) -> u64 {
        todo!()
    }
}