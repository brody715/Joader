use crossbeam::channel::Sender;
use std::collections::HashMap;

use crate::proto::dataset::create_dataset_request::FileType as ProtoFileType;
use crate::proto::dataset::{CreateDatasetRequest, DataItem};
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

#[derive(Clone, Default, Debug)]
pub struct Dataset {
    items: Vec<DataItem>,
    dataset_type: DatasetType,
}

impl Dataset {
    pub fn get_type(&self) -> DatasetType {
        self.dataset_type.clone()
    }

    pub fn get_name(&self) -> &str {
        match &self.dataset_type {
            DatasetType::FileSystem(s) | DatasetType::LMDB(s) => s,
            DatasetType::None => panic!(),
        }
    }

    pub fn from_proto(req: CreateDatasetRequest) -> Dataset {
        let name: String = req.name.into();
        let mut dataset_type = DatasetType::FileSystem(name.clone());
        if req.r#type == ProtoFileType::Lmdb as i32 {
            dataset_type = DatasetType::LMDB(name);
        }
        Dataset {
            items: req.items,
            dataset_type,
        }
    }

    pub fn get(&self, idx: usize) -> DataItem {
        self.items[idx].clone()
    }
}

#[derive(Clone, Default)]
pub struct DatasetTable {
    dataset_table: HashMap<String, Dataset>,
}

impl DatasetTable {
    pub fn get(&self, idx: u32) -> &Dataset {
        todo!()
    }

    pub fn insert(&mut self, dataset: Dataset) -> Result<(), String> {
        let name = dataset.get_name();
        if self.dataset_table.contains_key(name) {
            return Err(format!("Dataset {:} has existed!", name));
        }
        log::info!("Insert dataset {:?}", dataset);
        self.dataset_table.insert(name.to_string(), dataset);
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<(), String> {
        if let None = self.dataset_table.remove(name) {
            return Err(format!("Dataset {:} is not existed!", name));
        };
        log::info!("Delete dataset {:?}", name);
        Ok(())
    }

    pub fn new() -> DatasetTable {
        DatasetTable {
            dataset_table: HashMap::new(),
        }
    }
}
