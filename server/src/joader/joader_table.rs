use std::collections::HashMap;

use crate::{
    cache::cache::Cache,
    loader::{Rloader, Sloader},
};

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
        log::info!("Add Joader {:?}", joader);
        let name = joader.get_name();
        if self.joader_table.contains_key(name) {
            return Err("Dataset has existed".into());
        }
        self.joader_table.insert(name.to_owned(), joader);
        Ok(())
    }
    pub fn del_joader(&mut self, name: &str) -> Result<(), String> {
        log::info!("Del joader {:?}", name);
        if let None = self.joader_table.remove(name) {
            return Err("Dataset has not existed".into());
        }
        Ok(())
    }

    pub fn get(&mut self, name: &str) -> Result<&mut Joader, String> {
        self.joader_table
            .get_mut(name)
            .ok_or("Joader has not existed".into())
    }

    pub fn add_loader(&mut self, loader: Sloader) -> Result<u64, String> {
        log::info!("Add Loader {:?}", loader);
        let joader = self.get(loader.get_name())?;
        joader.add(loader)?;
        Ok(joader.len())
    }

    pub fn del_loader(&mut self, loader: Rloader) -> Result<(), String> {
        log::info!("Del Loader {:?}", loader);
        self.get(loader.get_name())?.del(loader);
        Ok(())
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
            joader.next(&mut self.cache).await;
        }
    }
}
