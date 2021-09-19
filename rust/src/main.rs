#![feature(get_mut_unchecked)]
// mod loader;
mod proto;
mod service;
mod sampler;
mod task;
mod dataset;

// use crate::proto::{
//     task::{CreateTaskResponse, GetDataRespones},
//     task_grpc::{create_task, Task},
// };
// use crate::task::{Task as DNNTask, TaskManager};
// use futures::{FutureExt, TryFutureExt, channel::oneshot::Receiver};
// use grpcio::{Environment, ServerBuilder};
// use log::{error, info};
// use log4rs;
// use crossbeam::channel::{Receiver as crossReceiver, unbounded};
// use std::{collections::HashMap, io::{self, Read}, thread, sync::{Arc, Mutex}};

// #[derive(Clone)]
// struct TaskService<'a> {
//     task_manager: Arc<Mutex<TaskManager<'a>>>,
//     id: u64,
//     tasks: Arc<Mutex<HashMap::<u64, crossReceiver<u64>>>>
// }

// impl Task for TaskService<'_> {
//     fn create(
//         &mut self,
//         ctx: grpcio::RpcContext,
//         req: proto::task::CreateTaskRequest,
//         sink: grpcio::UnarySink<proto::task::CreateTaskResponse>,
//     ) {
//         // ctx.get
//         self.id += 1;
//         let (s,r) = unbounded();
//         let id = self.id;
        
//         let keys = req.get_keys().to_vec();
//         let weights = req.get_weights().to_vec();
//         let loader = req.get_loader().to_owned();
//         let family = req.get_family();
//         let dnn_task = DNNTask::new(id, keys, weights, loader, s, family.to_owned());
        
//         //lock()?
//         let mut task_manager = self.task_manager.lock().unwrap();
//         task_manager.add(dnn_task).unwrap();
//         let mut map = self.tasks.lock().unwrap();
//         map.insert(id, r);

//         let mut resp = CreateTaskResponse::default();
//         resp.set_task_id(id);
//         resp.set_path("/dev/shm/dlcache".to_string());
        
//         let f = sink
//             .success(resp)
//             .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e))
//             .map(|_| ());
//         ctx.spawn(f)
//     }

//     fn get_data(
//         &mut self,
//         ctx: grpcio::RpcContext,
//         req: proto::task::GetDataRequest,
//         sink: grpcio::UnarySink<proto::task::GetDataRespones>,
//     ) {
//         let task_id = req.get_task_id();
//         let mut resp = GetDataRespones::new();

//         //lock()?
//         let map = self.tasks.lock().unwrap();
//         let address = map[&task_id].recv().unwrap();

//         resp.set_address(address);
//         let f = sink
//             .success(resp)
//             .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e))
//             .map(|_| ());
//         ctx.spawn(f)
//     }
// }

// fn main() {
//     log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
//     info!("INFO");
//     let task_manager = Arc::new(Mutex::new(TaskManager::new()));
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

}