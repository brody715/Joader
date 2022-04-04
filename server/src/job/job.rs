use crossbeam::queue::ArrayQueue;
use std::sync::Arc;
// Loader store the information of schema, dataset and filter

#[derive(Debug)]
pub struct Job {
    id: u64,
    queue: ArrayQueue<Arc<Vec<u8>>>
}

impl Job {
    pub fn new(id: u64) -> Self {
        Job {
            id,
            queue: ArrayQueue::new(1024),
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn is_full(&self) -> bool {
        self.queue.is_full()
    }

    pub fn push(&self, v: Arc<Vec<u8>>) {
        self.queue.push(v);
    }

    pub fn get(&self) -> Option<Arc<Vec<u8>>> {
        self.queue.pop()
    }
}
