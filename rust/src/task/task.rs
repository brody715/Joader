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

impl TaskRef {
    pub fn new(id: u64, dataset_id: u32, keys: &[u32], sender: Sender<u64>) -> Self {
        TaskRef(Arc::new(Task {
            id,
            dataset_id,
            keys: keys.to_owned(),
            sender,
        }))
        
    }

    pub fn task(&self) -> &Task {
        &self.0
    }

    pub fn id(&self) -> &u64 {
        &self.0.id
    }

    pub fn dataset(&self) -> u32 {
        self.0.dataset_id.clone()
    }

    pub fn keys(&self) -> &Vec<u32> {
        &self.0.keys
    }

    pub fn sender(&self) -> Sender<u64> {
        self.0.sender.clone()
    }

    pub fn send(&self, address: u64) {
        self.0.sender.send(address).unwrap();
    }

    pub fn len(&self) -> usize {
        return self.0.keys.len()
    }
}