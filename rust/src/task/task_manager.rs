use crate::sampler::SamplerManager;
use crossbeam::channel::Sender;
use std::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct Task {
    id: u64,
    keys: Vec<String>,
    weights: Vec<i32>,
    loader: String,
    sender: Sender<u64>,
    family: String,
}

impl Task {
    pub fn new(
        id: u64,
        keys: Vec<String>,
        weights: Vec<i32>,
        loader: String,
        sender: Sender<u64>,
        family: String
    ) -> Task {
        Task {
            id,
            keys,
            weights,
            loader,
            sender,
            family
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
    
    pub fn weights(&self) -> &Vec<i32> {
        &self.weights
    }

    pub fn keys(&self) -> &Vec<String> {
        &self.keys
    }

    pub fn family(&self) -> &str {
        self.family.as_str()
    }

    pub fn sender(&self) -> Sender<u64> {
        self.sender.clone()
    }

    pub fn send(&self, address: u64) {
        self.sender.send(address).unwrap();
    }
}

#[derive(Clone)]
pub struct TaskManager<'a> {
    tasks: HashMap<u64, Task>,
    sampler_manager: SamplerManager<'a>,
}

impl TaskManager<'_> {
    pub fn new() -> Self {
        let sampler_manager = SamplerManager::new();
        TaskManager {
            tasks: HashMap::<u64, Task>::new(),
            sampler_manager,
        }
    }

    pub fn add(&mut self, task: Task) -> Result<(), ()> {
        if self.tasks.contains_key(&task.id) {
            return Err(());
        }
        self.tasks.insert(task.id, task);
        self.sampler_manager.add(&task);
        Ok(())
    }

    pub fn start_sample(task_manager: Arc<Mutex<Self>>) {
        loop {
            let mut task_manager = task_manager.lock().unwrap();
            //todo(xj): when sample manager is empty, the thread should be blocked util new tasks add
            task_manager.sampler_manager.sample();
        }
    }
}
