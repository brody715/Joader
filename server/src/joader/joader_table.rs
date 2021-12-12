use std::collections::HashMap;

use crate::{cache::cache::Cache, proto::distributed::SampleResult};

use super::joader::Joader;

#[derive(Debug)]
pub struct JoaderTable {
    // Joader is hash by the name of dataset
    joader_table: HashMap<u32, Joader>,
    cache: Cache,
}

impl JoaderTable {
    pub fn new(cache: Cache) -> JoaderTable {
        JoaderTable {
            joader_table: HashMap::new(),
            cache,
        }
    }

    pub fn add_joader(&mut self, joader: Joader) {
        log::debug!("Add Joader {:?}", joader.get_id());
        let id = joader.get_id();
        self.joader_table.insert(id, joader);
    }

    pub fn del_joader(&mut self, id: u32) {
        log::debug!("Del joader {:?}", id);
        self.joader_table.remove(&id);
    }

    pub fn get_mut(&mut self, id: u32) -> &mut Joader {
        self.joader_table.get_mut(&id).unwrap()
    }

    pub fn get_shm_path(&self) -> String {
        self.cache.get_shm_path().to_string()
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
                joader.next(&mut self.cache).await;
            }
        }
    }

    pub fn set_hash_key(&mut self, num: u32) {
        for (_, v) in self.joader_table.iter_mut() {
            v.set_hash_key(num);
        }
    }

    pub fn remote_read(&mut self, _sample_res: &Vec<SampleResult>) {
        todo!()
    }
}
