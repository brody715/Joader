use std::collections::HashMap;

use crossbeam::channel::Sender;
use crate::{sampler::SamplerManager, task::TaskRef};

#[derive(Clone)]
pub struct TaskManager {
    task_table: HashMap<u64, TaskRef>,
    send_task: Sender<TaskRef>
}

impl TaskManager {
    pub fn new(sender: Sender<TaskRef>) -> Self {
        let sampler_manager = SamplerManager::new();
        TaskManager {
            task_table: HashMap::<u64, TaskRef>::new(),
            send_task: sender
        }
    }

    pub fn add(&mut self, task: TaskRef) -> Result<(), ()> {
        if self.task_table.contains_key(&task.id()) {
            return Err(());
        }
        self.task_table.insert(*task.id(), task.clone());
        self.send_task.send(task).unwrap();
        Ok(())
    }
}
