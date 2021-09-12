use std::collections::HashMap;
use sampler::Sampler;
use crate::task::Task;

use super::sampler;

#[derive(Clone)]
pub struct SamplerManager<'a> {
    sampler_table: HashMap<&'a str, Sampler<'a>>,
}

impl SamplerManager<'_> {
    pub fn new() -> Self {
        SamplerManager {
            sampler_table: HashMap::<&str, Sampler>::new(),
        }
    }

    pub fn add(&mut self, task: &Task) {
        // only support filesystem currently
        self.sampler_table[task.family()].insert(task.id(), task.weights(), task.keys());
    }

    pub fn sample(&mut self) {
        todo!()
    }
}
