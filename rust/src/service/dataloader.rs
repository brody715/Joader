// use crate::proto::dataloader::CreateDataloaderResponse;
// use crate::proto::dataloader_grpc::DataLoader;
// use crate::task::{TaskManager, TaskRef};
// use crossbeam::channel::{unbounded, Receiver};
// use futures::FutureExt;
// use proto::dataloader::{LoaderStatus, NextResponse};
// use std::collections::HashMap;
// use std::sync::atomic::{AtomicU64, Ordering};
// use std::sync::{Arc, Mutex};
// pub struct LoadTaskServiceImpl {
//     task_mgr: Arc<Mutex<TaskManager>>,
//     id: AtomicU64,
//     task_rev: Arc<Mutex<HashMap<u64, Receiver<u64>>>>,
// }

// impl DataLoader for LoadTaskServiceImpl {
//     fn create_dataloader(
//         &mut self,
//         ctx: grpcio::RpcContext,
//         req: crate::proto::dataloader::CreateDataloaderRequest,
//         sink: grpcio::UnarySink<crate::proto::dataloader::CreateDataloaderResponse>,
//     ) {
//         self.id.fetch_add(1, Ordering::SeqCst);
//         let id = self.id.load(Ordering::SeqCst);
//         let (sx, rx) = unbounded::<u64>();
//         let task = TaskRef::new(id, req.get_dataset_id(), req.get_keys(), sx);

//         let mut task_env = self.task_rev.lock().unwrap();
//         task_env.insert(id, rx);

//         let mut mgr = self.task_mgr.lock().unwrap();
//         let mut resp = CreateDataloaderResponse::default();
//         resp.set_loader_id(id);
//         let f = sink.success(resp).map(|_| ());
//         ctx.spawn(f);
//     }

//     fn next(
//         &mut self,
//         ctx: grpcio::RpcContext,
//         req: crate::proto::dataloader::NextRequest,
//         sink: grpcio::UnarySink<crate::proto::dataloader::NextResponse>,
//     ) {
//         let id = req.get_loader_id();
//         let mut recv = self.task_rev.lock().unwrap();
//         let addr = recv.get(&id).unwrap().recv().unwrap();
//         let mut resp = NextResponse::default();
//         resp.set_address(addr);
//         let f = sink.success(resp).map(|_| ());
//         ctx.spawn(f);
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


#[cfg(test)]
mod tests {
}