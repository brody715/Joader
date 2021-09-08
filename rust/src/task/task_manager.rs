use std::os::unix::thread;
use std::sync::mpsc::channel;
use crate::sampler::{self, Sampler, SamplerManager};


#[derive(Clone)]
pub struct Task {
    id: u64,
    keys: Vec<String>,
    weights: Vec<i32>
}

impl Task {
    pub fn new(id: u64, keys: Vec<String>, weights: Vec<i32>) -> Self {
        Task{id, keys, weights}
    }
}

#[derive(Clone)]
pub struct TaskManager {
    tasks: Vec<Task>,
    id: u64,
    sampler_manager: SamplerManager
    
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {tasks: Vec::<Task>::new(), id: 0, sampler_manager: SamplerManager{} }
    }

    pub fn new_id(&mut self) -> u64 {
        let id = self.id;
        id
    }

    pub fn add(&mut self, task: Task) -> Result<(), ()> {
        self.tasks.push(task);
        Ok(())
    }

    pub fn append_sampler(&mut self) {
        // let (tx, rx) = channel::<&str>();
        // thread::Spawn(move|| {
        //     self.sampler_manager.start()
        // });
        todo!()
    }
}