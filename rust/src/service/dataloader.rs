// use std::sync::{Arc, Mutex};

// use crate::proto::dataloader::{CreateDataloaderRequest, CreateDataloaderResponse};
// use crate::proto::dataloader_grpc::DataLoader;
// use crate::task::task_manager;
// #[derive(Clone)]
// pub struct TaskServiceImpl {
//     mgr: Arc<Mutex<TaskManager>>,
//     id: u32,
// }

// impl DataLoader for TaskServiceImpl {
//     fn create_dataloader(
//         &mut self,
//         ctx: grpcio::RpcContext,
//         req: crate::proto::dataloader::CreateDataloaderRequest,
//         sink: grpcio::UnarySink<crate::proto::dataloader::CreateDataloaderResponse>,
//     ) {
//         todo!()
//     }

//     fn next(
//         &mut self,
//         ctx: grpcio::RpcContext,
//         req: crate::proto::dataloader::NextRequest,
//         sink: grpcio::UnarySink<crate::proto::dataloader::NextResponse>,
//     ) {
//         todo!()
//     }

//     fn delete_dataloader(
//         &mut self,
//         ctx: grpcio::RpcContext,
//         req: crate::proto::dataloader::DeleteDataloaderRequest,
//         sink: grpcio::UnarySink<crate::proto::dataloader::DeleteDataloaderResponse>,
//     ) {
//         todo!()
//     }
// }
