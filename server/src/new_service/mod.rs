mod dataset_svc;
use std::{sync::Arc, collections::HashMap};

pub use dataset_svc::*;
mod job_svc;
pub use job_svc::*;
mod common;
pub use common::*;
mod distributed_svc;
pub use distributed_svc::*;
use tokio::sync::Mutex;

pub type IDTable = Arc<Mutex<HashMap<String, usize>>>;