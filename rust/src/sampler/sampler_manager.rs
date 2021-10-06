use super::{Sampler};
use crate::{dataset::DatasetTable, dataset::DataRequest, task::TaskRef};
use std::{collections::HashMap, sync::Arc};

// // a sampler has a dataset
#[derive(Clone)]
pub struct SamplerManager {
    sampler_table: HashMap<u32, Sampler>,
}

impl SamplerManager {
    pub fn new() -> Self {
        SamplerManager {
            sampler_table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, task: TaskRef) {
        if let Some(sampler) = self.sampler_table.get_mut(&task.dataset()) {
            sampler.insert(task);
        } else {
            let mut sampler = Sampler::new();
            sampler.insert(task.clone());
            self.sampler_table.insert(task.dataset(), sampler);
        }
    }

    pub fn sample(&mut self, dataset_table: &DatasetTable) -> Vec<DataRequest> {
        let mut res = Vec::new();
        for (dataset, sampler) in &mut self.sampler_table {
            let sampling_table = sampler.sample();
            for (data, set) in &sampling_table {
                res.push(DataRequest {
                    sender: set.iter().map(|x| x.sender().clone()).collect::<Vec<_>>(),
                    key: dataset_table.get(*dataset).get(*data as usize),
                    dataset: dataset_table.get(*dataset).get_type(),
                })
            }
        }
        res
    }
}
