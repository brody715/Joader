mod filesystem;
mod lmdb;
pub use lmdb::*;
pub use filesystem::*;
mod dummy;
use crate::cache::cache::Cache;
use crate::proto::dataset::{create_dataset_request::Type, CreateDatasetRequest};
pub use dummy::*;
use std::{fmt::Debug, sync::Arc};
pub trait Dataset: Sync + Send + Debug {
    fn get_name(&self) -> &str;
    fn get_indices(&self) -> Vec<u32>;
    fn read(&self, cache: &mut Cache, idx: u32, ref_cnt: usize) -> u64;
}
pub type DatasetRef = Arc<dyn Dataset>;

pub fn build_dataset(request: CreateDatasetRequest) -> DatasetRef {
    let t: Type = unsafe { std::mem::transmute(request.r#type) };
    match t {
        Type::Dummy => dummy::from_proto(request),
        Type::Filesystem => filesystem::from_proto(request),
        Type::Lmdb => lmdb::from_proto(request),
    }
}
