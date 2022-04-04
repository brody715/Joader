use crate::new_dataset::{DatasetRef};
use crate::job::Job;
use threadpool::ThreadPool;
use crate::sampler_bitmap::sampler_tree::SamplerTree;
use crate::local_cache::cache::Cache;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::Mutex;
#[derive(Debug)]
pub struct Joader {
    dataset: DatasetRef,
    sampler_tree: Arc<Mutex<SamplerTree>>,
    // map loader id to loader
    job_table: HashMap<u64, Arc<Job>>,
    ref_table: HashMap<u32, usize>,
}


fn read(idx: u32, ref_cnt: usize, cache: Arc<Mutex<Cache>>, dataset: DatasetRef, job_set: Vec<Arc<Job>>) {
    let data = dataset.read(idx);
    let mut cache_lock = cache.lock().unwrap();
    let key = dataset.get_id().to_string() + &idx.to_string();
    cache_lock.set(&key, data.clone(), ref_cnt);
    for job in job_set {
        job.push(data.clone());
    }
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
        let sampler_tree = Arc::new(Mutex::new(SamplerTree::new()));
        let joader = Joader {
            dataset,
            sampler_tree: sampler_tree.clone(),
            job_table: HashMap::new(),
            ref_table,
        };
        joader
    }

    
    pub async fn next(&mut self, pool: &mut ThreadPool, cache: Arc<Mutex<Cache>>) {
        let mut mask = HashSet::new();
        for (id, job) in self.job_table.iter() {
            if job.is_full() {
                mask.insert(*id);
            }
        }
        let sample_res = {
            let mut sampler_tree_lock = self.sampler_tree.lock().unwrap();
            sampler_tree_lock.sample(&mask)
        };
        for (data_idx, job_id_set) in sample_res {
            let ref_cnt = self.get_ref_cnt(data_idx, job_id_set.len());
            let dataset = self.dataset.clone();
            let clone_cache = cache.clone();
            let mut job_set = Vec::new();
            for job_id in job_id_set {
                job_set.push(self.job_table[&job_id].clone());
            }
            pool.execute(move || read(data_idx, ref_cnt, clone_cache, dataset, job_set))
        }
    }

    pub fn del_loader(&mut self, id: u64) {
        log::debug!("Del loader {}", id);
        let mut sampler_tree = self.sampler_tree.lock().unwrap();
        let valuse = sampler_tree.get_loader_values(id);
        sampler_tree.delete(id);
        // Todo(xj): clear cache
        for v in valuse.iter() {
            *self.ref_table.get_mut(v).unwrap() -= 1;
        }
        self.job_table.remove(&id);
    }

    pub fn add_loader(&mut self, loader_id: u64, nums: u32) {
        log::debug!("Add a loader {} at {}", loader_id, self.dataset.get_id());
        self.job_table
            .insert(loader_id, Arc::new(Job::new(loader_id)));
        for (_, cnt) in self.ref_table.iter_mut() {
            *cnt += 1;
        }
    }

    pub fn get_id(&self) -> u32 {
        self.dataset.get_id()
    }

    pub fn is_empty(&self) -> bool {
        self.job_table.is_empty()
    }

    pub fn len(&self) -> u64 {
        self.dataset.len()
    }
}