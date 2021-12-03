use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::proto::dataloader::CreateDataloaderRequest;
// Loader store the information of schema, dataset and filter
#[derive(Default, Debug, Clone)]
struct Loader {
    dataset_name: String,
    id: u64,
}

#[derive(Debug)]
pub struct Sloader {
    loader: Loader,
    s: UnboundedSender<u64>,
}
#[derive(Debug)]
pub struct Rloader {
    loader: Loader,
    r: UnboundedReceiver<u64>,
}
pub fn from_proto(request: CreateDataloaderRequest, id: u64) -> (Sloader, Rloader) {
    let loader = Loader {
        dataset_name: request.name,
        id,
    };
    let (s, r) = mpsc::unbounded_channel::<u64>();
    (
        Sloader {
            loader: loader.clone(),
            s,
        },
        Rloader { loader, r },
    )
}

impl Rloader {
    pub async fn next(&mut self) -> u64 {
        self.r.recv().await.unwrap()
    }

    pub fn get_id(&self) -> u64 {
        self.loader.id
    }

    pub fn get_name(&self) -> &str {
        &self.loader.dataset_name
    }
}

impl Sloader {
    pub fn get_id(&self) -> u64 {
        self.loader.id
    }

    pub fn send(&self, addr: u64) {
        self.s.send(addr).unwrap();
    }

    pub fn get_name(&self) -> &str {
        &self.loader.dataset_name
    }
}
