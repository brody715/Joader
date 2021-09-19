// use crate::sampler::SamplerManager;
use crossbeam::channel::Sender;
use std::borrow::Borrow;
use std::sync::Mutex;
use std::{collections::HashMap, sync::Arc};
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};

pub struct Task {
    id: u64,
    dataset_id: u32,
    keys: Vec<u32>,
    sender: Sender<u64>,
}

#[derive(Clone)]
pub struct TaskRef(Arc<Task>);

impl Hash for TaskRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.id.hash(state);
    }
}

impl PartialEq for TaskRef {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id
    }
}

impl Eq for TaskRef {}

impl Borrow<u64> for TaskRef {
    fn borrow(&self) -> &u64 {
        self.0.id()
    }
}

impl Task {
    pub fn new(id: u64, dataset_id: u32, keys: Vec<u32>, sender: Sender<u64>) -> Task {
        Task {
            id,
            dataset_id,
            keys,
            sender,
        }
    }

    pub fn id(&self) -> &u64 {
        &self.id
    }

    pub fn dataset(&self) -> u32 {
        self.dataset_id
    }

    pub fn keys(&self) -> &Vec<u32> {
        &self.keys
    }

    pub fn sender(&self) -> Sender<u64> {
        self.sender.clone()
    }

    pub fn send(&self, address: u64) {
        self.sender.send(address).unwrap();
    }

    pub fn len(&self) -> usize {
        return self.keys.len()
    }
}

#[derive(Clone)]
pub struct TaskManager {}

// impl TaskManager<'_> {
//     pub fn new() -> Self {
//         let sampler_manager = SamplerManager::new();
//         TaskManager {
//             tasks: HashMap::<u64, Task>::new(),
//             sampler_manager,
//         }
//     }

//     pub fn add(&mut self, task: Task) -> Result<(), ()> {
//         if self.tasks.contains_key(&task.id) {
//             return Err(());
//         }
//         self.tasks.insert(task.id, task.clone());
//         self.sampler_manager.add(&self.tasks[&task.id]);
//         Ok(())
//     }

//     pub fn start_sample(task_manager: Arc<Mutex<Self>>) {
//         loop {
//             let mut task_manager = task_manager.lock().unwrap();
//             //todo(xj): when sample manager is empty, the thread should be blocked util new tasks add
//             task_manager.sampler_manager.sample();
//         }
//     }
// }
