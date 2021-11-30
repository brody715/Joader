use tokio::sync::mpsc::UnboundedSender;

use crate::{dataset::DatasetRef, loader::Sloader};

pub struct Joader {}

impl Joader {
    pub fn new(dataset: DatasetRef) -> Joader {
        todo!()
    }

    pub fn next(&mut self) {
        todo!()
    }

    pub fn insert(&mut self) {
        todo!()
    }

    pub fn add(&mut self, s: Sloader) -> Result<u64, String> {
        // Id > 0
        todo!()
    }
}
