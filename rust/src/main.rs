use crate::task;
use grpcio::{Environment, Result, ServerBuilder, ShutdownFuture};
use std::sync::{Arc, Mutex};


pub struct Server {
    rpc_srv: grpcio::Server,
}

impl Server {
  pub fn new() -> Result<Server> {
    let grpc_srv = ServerBuilder::new(Arc::new(Environment::new(1)))
      .bind("0.0.0.0", 5688)
      .register_service(create_task_service(task_srv))
      .register_service(create_exchange_service(exchange_srv))
      .build()?;
    Ok(Server { rpc_srv: grpc_srv })
  }

  pub fn start(&mut self) {
    self.rpc_srv.start();
  }

  pub fn shutdown(&mut self) -> ShutdownFuture {
    self.rpc_srv.shutdown()
  }
}
fn main() {
    let server = Server::new();
}
