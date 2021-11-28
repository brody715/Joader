use crate::proto::common::{status::Code as RspCode, Status as RspStatus};

pub fn to_status(result: Result<(), String>) -> RspStatus {
    match result {
        Err(msg) => RspStatus {
            code: RspCode::Err as i32,
            msg,
        },
        Ok(_) => RspStatus {
            code: RspCode::Ok as i32,
            msg: "succ".into(),
        },
    }
}
