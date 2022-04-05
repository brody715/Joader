use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
#[derive(Debug, Clone)]
pub struct IdGenerator {
    dataset_id: Arc<AtomicUsize>,
    loader_id: Arc<AtomicUsize>,
}

impl IdGenerator {
    pub async fn get_dataset_id(&self) -> usize {
        self.dataset_id.fetch_add(1, Ordering::SeqCst);
        self.dataset_id.load(Ordering::SeqCst)
    }

    pub async fn get_loader_id(&self) -> usize {
        self.loader_id.fetch_add(1, Ordering::SeqCst);
        self.loader_id.load(Ordering::SeqCst)
    }

    pub fn new() -> Self {
        Self {
            dataset_id: Arc::new(AtomicUsize::new(0)),
            loader_id: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn parse_dataset_id(loader_id: u64) -> u32 {
        (loader_id >> 32) as u32
    }
}
