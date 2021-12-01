use crate::{dataset::DatasetRef, loader::{Rloader, Sloader}};

pub struct Joader {}

impl Joader {
    pub fn new(dataset: DatasetRef) -> Joader {
        todo!()
    }

    pub fn next(&mut self) {
        todo!()
    }

    pub fn del(&mut self, r: Rloader) {
        todo!()
    }

    pub fn add(&mut self, s: Sloader) -> Result<u64, String> {
        // Id > 0
        todo!()
    }
}
