use std::collections::HashMap;

use crate::cache::cache::Cache;

use super::joader::Joader;

#[derive(Debug)]
pub struct JoaderTable {
    // Joader is hash by the name of dataset
    joader_table: HashMap<String, Joader>,
    cache: Cache,
}

impl JoaderTable {
    pub fn new(cache: Cache) -> JoaderTable {
        JoaderTable {
            joader_table: HashMap::new(),
            cache,
        }
    }

    pub fn add_joader(&mut self, joader: Joader) -> Result<(), String> {
        log::debug!("Add Joader {:?}", joader.get_name());
        let name = joader.get_name();
        if self.joader_table.contains_key(name) {
            return Err("Dataset has existed".into());
        }
        self.joader_table.insert(name.to_owned(), joader);
        Ok(())
    }

    pub fn del_joader(&mut self, name: &str) -> Result<(), String> {
        log::debug!("Del joader {:?}", name);
        if let None = self.joader_table.remove(name) {
            return Err("Dataset has not existed".into());
        }
        Ok(())
    }

    pub fn get_mut(&mut self, name: &str) -> Result<&mut Joader, String> {
        self.joader_table
            .get_mut(name)
            .ok_or("Joader has not existed".into())
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
}
