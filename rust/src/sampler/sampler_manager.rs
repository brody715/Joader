// use std::collections::HashMap;
// use sampler::Sampler;
// use crate::task::Task;

// use super::sampler;

// // a sampler has a dataset
// #[derive(Clone)]
pub struct SamplerManager {}

// impl SamplerManager<'_> {
//     pub fn new() -> Self {
//         SamplerManager {
//             sampler_table: HashMap::<&str, Sampler>::new(),
//         }
//     }

//     pub fn add(&mut self, task: &Task) {
//         // only support filesystem currently
//         // self.sampler_table[task.family()].insert(task.id(), task.weights(), task.keys());
//         todo!()
//     }

//     pub fn sample(&mut self) {
//         todo!()
//     }
// }
