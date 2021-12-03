use crate::proto::common::{status::Code as RspCode, Status as RspStatus};

pub fn to_status<T>(result: &Result<T, String>) -> RspStatus {
    match result {
        Err(msg) => RspStatus {
            code: RspCode::Err as i32,
            msg: msg.to_string(),
        },
        Ok(_) => RspStatus {
            code: RspCode::Ok as i32,
            msg: "succ".into(),
        },
    }
}
