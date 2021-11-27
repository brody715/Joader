#![feature(get_mut_unchecked)]
// #![feature(ptr_metadata)]
use std::{
    sync::{Arc, Mutex},
    thread,
};
use crossbeam::channel::unbounded;
use dataset::{DatasetTable, DataRequest};
use sampler::SamplerManager;
use task::{TaskManager, TaskRef};
// mod loader;
mod dataset;
mod loader;
mod sampler;
mod task;
mod cache;
mod proto;
// fn main() {
//
//     info!("INFO");
//
//     let task_srv = TaskService {
//         task_manager:task_manager.clone(),
//         id: 0,
//         tasks: Arc::new(Mutex::new(HashMap::<u64, crossReceiver<u64>>::new()))
//     };
//     let mut server = ServerBuilder::new(Arc::new(Environment::new(1)))
//         .bind("0.0.0.0", 5688)
//         .register_service(create_task(task_srv))
//         .build()
//         .unwrap();
//     server.start();
//     let sampler = thread::spawn(move || {
//         TaskManager::start_sample(task_manager);
//     });
//     sampler.join().unwrap();
// }

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let (task_sx, task_rx) = unbounded::<TaskRef>();
    let (request_sx, request_rx) = unbounded::<DataRequest>();
    let mut task_manager = Arc::new(Mutex::new(TaskManager::new(task_sx)));
    let mut dataset_table = Arc::new(DatasetTable::new());
    let mut sampler_manager = SamplerManager::new();

    loop {
        let requests = sampler_manager.sample(dataset_table.as_ref());
        while !task_rx.is_empty() || requests.is_empty() {
            sampler_manager.insert(task_rx.recv().unwrap())
        }
        for request in requests {
            request_sx.send(request).unwrap();
        }
    }
}
