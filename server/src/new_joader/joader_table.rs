use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
// casue aysnc trait has not been supported, we use thread pool
use super::joader::Joader;
use crate::local_cache::cache::Cache;
use std::sync::Mutex;
use threadpool::ThreadPool;

#[derive(Debug)]
pub struct JoaderTable {
    // Joader is hash by the name of dataset
    joader_table: HashMap<u32, Joader>,
    cache: Arc<Mutex<Cache>>,
    pool: ThreadPool,
}

impl JoaderTable {
    pub fn new(cache: Arc<Mutex<Cache>>) -> JoaderTable {
        JoaderTable {
            joader_table: HashMap::new(),
            cache,
            pool: ThreadPool::new(32),
        }
    }

    pub fn add_joader(&mut self, mut joader: Joader) {
        log::debug!("Add Joader {:?}", joader.get_id());
        let id = joader.get_id();
        self.joader_table.insert(id, joader);
    }

    pub fn del_joader(&mut self, id: u32) {
        log::debug!("Del joader {:?}", id);
        self.joader_table.remove(&id);
    }

    pub fn get_mut(&mut self, id: u32) -> &mut Joader {
        log::debug!("Get joader {:?}", id);
        self.joader_table.get_mut(&id).unwrap()
    }

    pub fn is_empty(&self) -> bool {
        let mut empty = true;
        for (_, joader) in self.joader_table.iter() {
            empty &= joader.is_empty();
        }
        empty
    }

    pub async fn next(&mut self) {
        for (_, joader) in self.joader_table.iter_mut() {
            if !joader.is_empty() {
                let res = joader.next(&mut self.pool, self.cache.clone()).await;
            }
        }
    }

    pub fn contains_dataset(&self, id: u32) -> bool {
        self.joader_table.contains_key(&id)
    }
}