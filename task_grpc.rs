// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_TASK_CREATE: ::grpcio::Method<super::task::CreateTaskRequest, super::task::CreateTaskResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Task/Create",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TASK_GET_DATA: ::grpcio::Method<super::task::GetDataRequest, super::task::GetDataRespones> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Task/GetData",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct TaskClient {
    client: ::grpcio::Client,
}

impl TaskClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        TaskClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn create_opt(&self, req: &super::task::CreateTaskRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::task::CreateTaskResponse> {
        self.client.unary_call(&METHOD_TASK_CREATE, req, opt)
    }

    pub fn create(&self, req: &super::task::CreateTaskRequest) -> ::grpcio::Result<super::task::CreateTaskResponse> {
        self.create_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_async_opt(&self, req: &super::task::CreateTaskRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::task::CreateTaskResponse>> {
        self.client.unary_call_async(&METHOD_TASK_CREATE, req, opt)
    }

    pub fn create_async(&self, req: &super::task::CreateTaskRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::task::CreateTaskResponse>> {
        self.create_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_data_opt(&self, req: &super::task::GetDataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::task::GetDataRespones> {
        self.client.unary_call(&METHOD_TASK_GET_DATA, req, opt)
    }

    pub fn get_data(&self, req: &super::task::GetDataRequest) -> ::grpcio::Result<super::task::GetDataRespones> {
        self.get_data_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_data_async_opt(&self, req: &super::task::GetDataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::task::GetDataRespones>> {
        self.client.unary_call_async(&METHOD_TASK_GET_DATA, req, opt)
    }

    pub fn get_data_async(&self, req: &super::task::GetDataRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::task::GetDataRespones>> {
        self.get_data_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Task {
    fn create(&mut self, ctx: ::grpcio::RpcContext, req: super::task::CreateTaskRequest, sink: ::grpcio::UnarySink<super::task::CreateTaskResponse>);
    fn get_data(&mut self, ctx: ::grpcio::RpcContext, req: super::task::GetDataRequest, sink: ::grpcio::UnarySink<super::task::GetDataRespones>);
}

pub fn create_task<S: Task + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TASK_CREATE, move |ctx, req, resp| {
        instance.create(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_TASK_GET_DATA, move |ctx, req, resp| {
        instance.get_data(ctx, req, resp)
    });
    builder.build()
}
