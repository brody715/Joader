use std::sync::Arc;

use tokio::sync::Mutex;

use crate::proto::common::{status::Code as RspCode, Status as RspStatus};

pub fn to_status<T>(result: &Result<T, String>) -> RspStatus {
    match result {
        Err(msg) => RspStatus {
            code: RspCode::Err as i32,
            msg: msg.to_string(),
        },
        Ok(_) => RspStatus {
            code: RspCode::Ok as i32,
            msg: "Success".into(),
        },
    }
}

#[inline]
pub fn succ() -> RspStatus {
    RspStatus {
        code: RspCode::Ok as i32,
        msg: "Success".into(),
    }
}

#[derive(Debug)]
pub struct GlobalID {
    id: Arc<Mutex<u64>>,
}

impl GlobalID {
    pub async fn get_id(&self) -> u64 {
        let mut id = self.id.lock().await;
        *id += 1;
        *id
    }

    pub fn new() -> GlobalID {
        GlobalID {
            id: Arc::new(Mutex::new(0)),
        }
    }
}
