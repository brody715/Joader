use std::collections::HashMap;
use crate::dataset::DatasetRef;
use crate::loader::{Rloader, Sloader};
use crate::sampler::sampler_tree::SamplerTree;

pub struct Joader {
    // dataset: DatasetRef,
    sampler: SamplerTree,
    // loader_table: HashMap<u64, Sloader>,
}


impl Joader {
    pub fn new(dataset: DatasetRef) -> Joader {
        Joader {
            // dataset,
            sampler: SamplerTree::new(),
            // loader_table: HashMap::new(),
        }
    }

    pub fn next(&mut self) {
        // let data_table = self.sampler.sample();

    }

    pub fn del(&mut self, r: Rloader) {
        todo!()
    }

    pub fn add(&mut self, s: Sloader) -> Result<u64, String> {
        // let len = self.loader_table.len();
        todo!()
    }
}
