use crate::loader::{Rloader, Sloader};

use super::joader::Joader;

#[derive(Debug, Default)]
pub struct JoaderTable {}

impl JoaderTable {
    pub fn add_joader(&mut self, joader: Joader) -> Result<(), String> {
        todo!()
    }
    pub fn del_joader(&mut self, name: &str) -> Result<(), String> {
        todo!()
    }

    pub fn get(&mut self, name: &str) -> Result<&mut Joader, String> {
        todo!()
    }


    pub fn add_loader(&mut self, loader: Sloader) -> Result<(), String> {
        todo!()
    }
    pub fn del_loader(&mut self, loader: Rloader) -> Result<(), String> {
        todo!()
    }
    pub fn get_shm_path(&self) -> String {
        todo!()
    }
}
