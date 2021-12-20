use crate::dataset::DatasetRef;
use crate::loader::{DataSender, Loader};
use crate::sampler::sampler_tree::SamplerTree;
use crate::{cache::cache::Cache, loader::IdxSender};
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

    pub fn set_hash_key(&mut self, key: u32) {
        self.key = key;
    }

    // pub fn get_mut(&mut self, id: u64) -> &mut Loader {
    //     self.loader_table.get_mut(&id).unwrap()
    // }

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

    #[inline]
    fn choose_local_host(&self, host_id: u32) -> bool {
        host_id == self.key - 1
    }

    async fn distributed(&mut self, data_idx: u32, loader_ids: &mut HashSet<u64>) {
        let host_id = self.get_hash_host(data_idx);
        if !self.choose_local_host(host_id) {
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
                        "Joader distribted data {:} to loader {:?} in host {:?}",
                        data_idx,
                        loader_id,
                        host_id
                    );
                    loader_ids.remove(&loader_id);
                }
            }
        }
    }

    pub async fn remote_read(
        &mut self,
        sampler_res: &HashMap<u32, HashSet<u64>>,
        cache: &mut Cache,
    ) {
        
        for (data_idx, loader_ids) in sampler_res {
            // Todo: Support remote ref_cnt
            let loader_cnt = loader_ids.len();
            let addr = self.dataset.read(cache, *data_idx, 0, loader_cnt);
            for (idx, id) in loader_ids.iter().enumerate() {
                log::debug!("Joader load data {:} at {:?} to {:?}", data_idx, addr, id);
                self.loader_table[id].send_data(addr, idx).await;
            }
        }
    }

    pub async fn next(&mut self, cache: &mut Cache) {
        self.clear_empty_loader().await;
        let mut data_table = self.sampler_tree.sample();
        for (data_idx, loader_ids) in data_table.iter_mut() {
            let ref_cnt = self.get_ref_cnt(*data_idx, loader_ids.len());
            self.distributed(*data_idx, loader_ids).await;
            let loader_cnt = loader_ids.len();
            if !loader_ids.is_empty() {
                let addr = self.dataset.read(cache, *data_idx, ref_cnt, loader_cnt);
                for (idx, id) in loader_ids.iter().enumerate() {
                    log::debug!("Joader load data {:} at {:?} to {:?}", data_idx, addr, id);
                    self.loader_table[id].send_data(addr, idx).await;
                }
            }
        }
    }

    pub fn del_loader(&mut self, id: u64) {
        let valuse = self.sampler_tree.get_loader_values(id);
        self.sampler_tree.delete(id);
        for v in valuse.iter() {
            *self.ref_table.get_mut(v).unwrap() -= 1;
        }
        self.loader_table.remove(&id);
    }

    pub fn add_idx_sender(&mut self, loader_id: u64, idx_sender: IdxSender, host_id: u64) {
        let loader = self.loader_table.get_mut(&loader_id).unwrap();
        loader.add_idx_sender(idx_sender, host_id);
        if loader.ready() {
            self.sampler_tree
                .insert(self.dataset.get_indices(), loader_id);
        }
    }

    pub fn add_data_sender(&mut self, loader_id: u64, data_sender: DataSender) {
        let loader = self.loader_table.get_mut(&loader_id).unwrap();
        loader.add_data_sender(data_sender);
        if loader.ready() {
            log::debug!("loader id {} ready", loader_id);
            self.sampler_tree
                .insert(self.dataset.get_indices(), loader_id);
        }
    }

    pub fn del_idx_sender(&mut self, loader_id: u64, host_id: u64) {
        let loader = self.loader_table.get_mut(&loader_id).unwrap();
        loader.del_idx_sender(host_id);
    }

    pub fn del_data_sender(&mut self, loader_id: u64) {
        let loader = self.loader_table.get_mut(&loader_id).unwrap();
        loader.del_data_sender();
    }

    pub fn is_loader_empty(&self, loader_id: u64) -> bool {
        self.loader_table[&loader_id].is_empty()
    }

    pub fn add_loader(&mut self, loader_id: u64, nums: u32) {
        log::debug!("Add a loader {}", loader_id);
        self.loader_table
            .insert(loader_id, Loader::new(loader_id, nums));
        for (_, cnt) in self.ref_table.iter_mut() {
            *cnt += 1;
        }
    }

    pub fn get_mut_loader(&mut self, id: u64) -> &mut Loader {
        self.loader_table.get_mut(&id).unwrap()
    }

    pub fn get_id(&self) -> u32 {
        self.dataset.get_id()
    }

    pub fn is_empty(&self) -> bool {
        let mut empty = true;
        for (_, l) in &self.loader_table {
            empty &= l.closed();
        }
        empty
    }

    pub fn len(&self) -> u64 {
        self.dataset.len()
    }
}
