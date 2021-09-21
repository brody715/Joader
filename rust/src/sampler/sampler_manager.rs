use std::{collections::HashMap, ops::Index, string, sync::Arc, thread};

use crate::{dataset::Dataset, task::TaskRef};

use super::Sampler;

// use std::collections::HashMap;
// use sampler::Sampler;
// use crate::task::Task;

// use super::sampler;

// // a sampler has a dataset
// #[derive(Clone)]
pub struct SamplerManager {
    sampler_table: HashMap<u32 , Sampler>
}

impl SamplerManager {
    pub fn new() -> Self {
        SamplerManager { sampler_table: HashMap::new() }
    }

    pub fn insert(&mut self, task: TaskRef) -> Result<(), ()> {
        if let Some(sampler) = self.sampler_table.get_mut(&task.dataset()) {
            sampler.insert(task);
        } else {
            return Err(());
        }
        Ok(())
    }

    pub fn create_dataset(&mut self, dataset: Arc<dyn Dataset>) {
        self.sampler_table.insert(dataset.id(), Sampler::new(dataset));
    }

    pub fn start_sample(&mut self) {
        for (_, sampler) in self.sampler_table {
            thread::spawn(move || &sampler.sample());
        }
    }
}