use crate::cache::cache::Cache;
use crate::dataset::DatasetRef;
use crate::loader::Loader;
use crate::sampler::sampler_tree::SamplerTree;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Joader {
    dataset: DatasetRef,
    sampler_tree: SamplerTree,
    // map loader id to loader
    loader_table: HashMap<u64, Loader>,
    ref_table: HashMap<u32, usize>,
    key: u32,
}

impl Joader {
    fn get_ref_cnt(&mut self, idx: u32, count: usize) -> usize {
        *self.ref_table.get_mut(&idx).unwrap() -= count;
        self.ref_table[&idx]
    }

    pub fn contains(&self, id: u64) -> bool {
        self.loader_table.contains_key(&id)
    }

    pub fn set_hash_key(&mut self, num: u32) {
        self.key = num + 1;
    }

    pub fn get_mut(&mut self, id: u64) -> Result<&mut Loader, String> {
        self.loader_table
            .get_mut(&id)
            .ok_or_else(|| format!("Loader {} does not existed!", id))
    }

    pub fn new(dataset: DatasetRef) -> Joader {
        let mut ref_table = HashMap::new();
        for i in dataset.get_indices() {
            ref_table.insert(i, 0);
        }
        Joader {
            dataset,
            sampler_tree: SamplerTree::new(),
            loader_table: HashMap::new(),
            ref_table,
            key: 1,
        }
    }

    #[inline]
    fn get_hash_host(&self, idx: u32) -> u32 {
        idx % self.key
    }

    pub async fn clear_empty_loader(&mut self) {
        let del_loader = self.sampler_tree.clear_loader();
        for id in del_loader {
            self.loader_table.get_mut(&id).unwrap().close().await;
            // We do not del loader there. Insteadly, the deletion is done by user
        }
    }

    async fn distributed(&mut self, data_idx: u32, loader_ids: &mut HashSet<u64>) {
        let host_id = self.get_hash_host(data_idx);
        if host_id != self.key - 1 {
            let loader_id_cloned = loader_ids.clone();
            for loader_id in loader_id_cloned {
                if self
                    .loader_table
                    .get_mut(&loader_id)
                    .unwrap()
                    .send_idx(data_idx, host_id as u64)
                    .await
                {
                    // we need distributed the idx to other hosts
                    log::debug!(
                        "Joader distribted data {:} to {:?} {:?}",
                        data_idx,
                        loader_id,
                        host_id
                    );
                    loader_ids.remove(&loader_id);
                }
            }
        }
    }

    pub async fn next(&mut self, cache: &mut Cache) {
        self.clear_empty_loader().await;
        let mut data_table = self.sampler_tree.sample();
        for (data_idx, loader_ids) in data_table.iter_mut() {
            let ref_cnt = self.get_ref_cnt(*data_idx, loader_ids.len());
            self.distributed(*data_idx, loader_ids).await;
            if !loader_ids.is_empty() {
                let addr = self.dataset.read(cache, *data_idx, ref_cnt);
                for id in loader_ids.iter() {
                    log::debug!("Joader load data {:} at {:?} to {:?}", data_idx, addr, id);
                    self.loader_table[id].send_data(addr).await;
                }
            }
        }
    }

    pub fn del(&mut self, id: u64) -> Result<(), String> {
        let valuse = self.sampler_tree.get_loader_values(id);
        self.sampler_tree.delete(id);
        for v in valuse.iter() {
            *self.ref_table.get_mut(v).unwrap() -= 1;
        }
        self.loader_table.remove(&id);
        Ok(())
    }

    pub fn add_loader(&mut self, loader_id: u64) {
        log::debug!("Add a loader {}", loader_id);
        self.loader_table.insert(loader_id, Loader::new(loader_id));
        self.sampler_tree
            .insert(self.dataset.get_indices(), loader_id);
        for (_, cnt) in self.ref_table.iter_mut() {
            *cnt += 1;
        }
    }

    pub fn get_mut_loader(&mut self, id: u64) -> &mut Loader {
        self.loader_table.get_mut(&id).unwrap()
    }

    pub fn get_name(&self) -> &str {
        self.dataset.get_name()
    }

    pub fn is_empty(&self) -> bool {
        self.loader_table.is_empty()
    }

    pub fn len(&self) -> u64 {
        self.dataset.len()
    }
}
