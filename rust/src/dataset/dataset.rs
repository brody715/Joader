use std::{collections::HashMap, rc::Rc};
use crossbeam::channel::Sender;

#[derive(Clone)]
pub enum DatasetType {
    FileSystem,
    LMDB(String),
}
pub struct DataRequest {
    pub sender: Vec<Sender<u64>>,
    pub key: DataItem,
    pub dataset: DatasetType
}

#[derive(Clone)]
pub struct DataItem {
    keys: Vec<String>
}

impl DataItem {
    pub fn new(keys: Vec<String>) -> Self {
        DataItem{keys}
    }

    pub fn keys(&self) -> &[String] {
        &self.keys
    }
}

#[derive(Clone)]
pub struct Dataset {
    dataset: Rc<Vec<DataItem>>,
    dataset_type: DatasetType
}

impl Dataset {
    pub fn get_type(&self) -> DatasetType {
        self.dataset_type.clone()
    }

    pub fn get(&self, idx: usize) -> DataItem {
        self.dataset[idx].clone()
    }
}

pub struct DatasetTable {
    dataset_table: HashMap<u32, Dataset>
}

impl DatasetTable {
    pub fn get(&self, idx: u32) -> &Dataset {
        &self.dataset_table.get(&idx).unwrap()
    }

    pub fn new() -> DatasetTable {
        DatasetTable { dataset_table: HashMap::new() }
    }
}