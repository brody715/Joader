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
    size: AtomicUsize,
}

impl Job {
    pub fn new(id: u64) -> (Arc<Self>, Receiver<Arc<Vec<Data>>>) {
        let (s, r) = channel::<Arc<Vec<Data>>>(CAP);
        (
            Arc::new(Job {
                id,
                sender: s,
                size: AtomicUsize::new(0),
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

    pub fn allocate(&self) -> bool {
        if self.sender.capacity() - self.size.load(Ordering::SeqCst) > 0 {
            self.size.fetch_add(1, Ordering::SeqCst);
            return true;
        }
        false
    }

    pub async fn push(&self, v: Arc<Vec<Data>>) {
        self.size.fetch_sub(1, Ordering::SeqCst);
        assert_eq!(self.is_full(), false);
        self.sender.send(v).await.unwrap();
    }
}
