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
    ref_table: HashMap<u32, usize>,
}

impl Joader {
    fn get_ref_cnt(&mut self, idx: u32, count: usize) -> usize {
        *self.ref_table.get_mut(&idx).unwrap() -= count;
        self.ref_table[&idx]
    }

    pub fn new(dataset: DatasetRef) -> Joader {
        let mut ref_table = HashMap::new();
        for i in dataset.get_indices() {
            ref_table.insert(i, 0);
        }
        Joader {
            dataset,
            sampler: SamplerTree::new(),
            loader_table: HashMap::new(),
            ref_table,
        }
    }

    pub async fn next(&mut self, cache: &mut Cache) {
        let data_table = self.sampler.sample();
        for (data, loader_ids) in &data_table {
            let ref_cnt = self.get_ref_cnt(*data, loader_ids.len());
            let addr = self.dataset.read(cache, *data, ref_cnt);
            for id in loader_ids {
                log::debug!("Joader send {:} to {:?}", addr, self.loader_table[id]);
                self.loader_table[id].send(addr).await;
            }
        }
    }

    pub fn del(&mut self, r: Rloader) {
        let id = r.get_id();
        let valuse = self.sampler.get_loader_values(id);
        for v in valuse.iter() {
            *self.ref_table.get_mut(v).unwrap() -= 1;
        }
        self.loader_table.remove(&id);
    }

    pub fn add(&mut self, s: Sloader) -> Result<u64, String> {
        let id = s.get_id();
        if self.loader_table.contains_key(&id) {
            return Err(format!("Loader {:?} has existed", id));
        }
        self.loader_table.insert(id, s);
        
        self.sampler.insert(self.dataset.get_indices(), id);
        log::debug!("Joader Insert {:?}: {:?} to {:?}", id, self.dataset.get_name(), self.sampler);
        for (_, cnt) in self.ref_table.iter_mut() {
            *cnt += 1;
        }
        Ok(id)
    }

    pub fn get_name(&self) -> &str {
        self.dataset.get_name()
    }

    pub fn is_empty(&self) -> bool {
        self.sampler.is_empty()
    }

    pub fn len(&self) -> u64 {
        self.dataset.len()
    }
}
