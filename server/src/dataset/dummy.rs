use super::Dataset;
use super::DatasetRef;
use crate::{
    cache::cache::Cache,
    proto::dataset::{CreateDatasetRequest, DataItem},
};
use std::{fmt::Debug, sync::Arc};

#[derive(Clone, Default, Debug)]
struct DummyDataset {
    _magic: u8,
    items: Vec<DataItem>,
    name: String,
}

pub fn new_dummy(len: usize, name: String) -> DatasetRef {
    let mut items = Vec::new();
    for i in 0..len {
        items.push(DataItem {
            keys: vec![i.to_string()],
        })
    }
    Arc::new(DummyDataset {
        _magic: 7u8,
        items,
        name,
    })
}

pub fn from_proto(request: CreateDatasetRequest) -> DatasetRef {
    let name = request.name;
    let items = request.items;
    Arc::new(DummyDataset {
        items,
        _magic: 7u8,
        name,
    })
}

fn _len() -> usize {
    256
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

    fn read(&self, _cache: &mut Cache, idx: u32, _ref_cnt: usize) -> u64 {
        idx as u64
    }

    fn len(&self) -> u64 {
        self.items.len() as u64
    }
}
