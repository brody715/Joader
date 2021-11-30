use tonic::codegen::http::request;

use crate::proto::dataloader::CreateDataloaderRequest;
// Loader store the information of schema, dataset and filter
#[derive(Default, Debug)]
pub struct Sloader {}
#[derive(Debug, Default)]
pub struct Rloader {}
pub fn from_proto(request: CreateDataloaderRequest) -> (Sloader, Rloader) {
    todo!()
}

impl Rloader {
    pub fn next(&mut self) -> u64 {
        todo!()
    }
}
