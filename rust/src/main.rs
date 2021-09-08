mod loader;
mod proto;
mod sampler;
mod task;

use log4rs;
use crate::proto::{
    task::{CreateTaskResponse, CreateTaskResponse_State as State},
    task_grpc::{create_task, Task},
};
use crate::task::{Task as DNNTask, TaskManager};
use futures::{channel::oneshot, executor::block_on, FutureExt, TryFutureExt};
use grpcio::{Environment, Server, ServerBuilder};
use log::{error, info};
use std::{io::{self, Read}, sync::Arc, thread};

#[derive(Clone)]
struct TaskService {}

impl Task for TaskService {
    fn create(
        &mut self,
        ctx: grpcio::RpcContext,
        req: proto::task::CreateTaskRequest,
        sink: grpcio::UnarySink<proto::task::CreateTaskResponse>,
    ) {
        // ctx.get
        let keys = req.get_keys().to_vec();
        let weights = req.get_weights().to_vec();
        let id = self.task_manager.new_id();
        let task = DNNTask::new(id, keys, weights);
        let mut resp = CreateTaskResponse::default();
        resp.set_name(id);
        match self.task_manager.add(task) {
            Ok(_) => resp.set_state(State::Ok),
            Err(_) => resp.set_state(State::False),
        }
        let f = sink
            .success(resp)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn heart_beat(
        &mut self,
        _ctx: grpcio::RpcContext,
        _req: proto::task::HeartBeatRequest,
        _sink: grpcio::UnarySink<proto::task::HeartBeatRespones>,
    ) {
        todo!()
    }
}

fn new_server() -> Server {
    let task_srv = TaskService::new();
    ServerBuilder::new(Arc::new(Environment::new(1)))
        .bind("0.0.0.0", 5688)
        .register_service(create_task(task_srv))
        .build()
        .unwrap()
}

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("INFO");
    let mut server = new_server();
    server.start();
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown());
}
