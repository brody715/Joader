use crate::cache::cache::Cache;
use crate::dataset::DatasetRef;
use crate::loader::{Rloader, Sloader};
use crate::sampler::sampler_tree::SamplerTree;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Joader {
    dataset: DatasetRef,
    sampler: SamplerTree,
    loader_table: HashMap<u64, Sloader>,
}

impl Joader {
    pub fn new(dataset: DatasetRef) -> Joader {
        Joader {
            dataset,
            sampler: SamplerTree::new(),
            loader_table: HashMap::new(),
        }
    }

    pub fn next(&mut self, cache: &mut Cache) {
        let data_table = self.sampler.sample();
        for (data, loader_ids) in &data_table {
            let addr = self.dataset.read(cache, *data);
            for id in loader_ids {
                self.loader_table[id].send(addr);
            }
        }
    }

    pub fn del(&mut self, r: Rloader) {
        self.loader_table.remove(&r.get_id());
    }

    pub fn add(&mut self, s: Sloader) -> Result<u64, String> {
        let id = s.get_id();
        self.loader_table.insert(id, s);
        self.sampler.insert(self.dataset.get_indices(), id);
        if self.loader_table.contains_key(&id) {
            return Err("Loader has existed".into());
        }
        Ok(id)
    }

    pub fn get_name(&self) -> &str {
        self.dataset.get_name()
    }

    pub fn is_empty(&self) -> bool {
        self.sampler.is_empty()
    }
}
