use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::proto::job::Data;
// Loader store the information of schema, dataset and filter
const CAP: usize = 1024;
#[derive(Debug)]
pub struct Job {
    id: u64,
    sender: Sender<Arc<Vec<Data>>>,
    capacity: AtomicUsize,
}

impl Job {
    pub fn new(id: u64) -> (Arc<Self>, Receiver<Arc<Vec<Data>>>) {
        let (s, r) = channel::<Arc<Vec<Data>>>(CAP);
        (
            Arc::new(Job {
                id,
                sender: s,
                capacity: AtomicUsize::new(CAP),
            }),
            r,
        )
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn is_full(&self) -> bool {
        self.sender.capacity() == 0
    }

    pub async fn push(&self, v: Arc<Vec<Data>>) {
        self.sender.send(v).await.unwrap();
        self.capacity.fetch_sub(1, Ordering::SeqCst);
    }

    pub fn can_push(&self) -> bool {
        if self.sender.capacity() <= self.capacity.load(Ordering::SeqCst) {
            return false;
        }
        self.capacity.fetch_add(1, Ordering::SeqCst);
        true
    }

    pub fn capacity(&self) -> usize {
        self.sender.capacity()
    }
}
