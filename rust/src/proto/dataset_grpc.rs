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

const METHOD_DATASET_CREATE_DATASET: ::grpcio::Method<super::dataset::CreateDatasetRequest, super::dataset::CreateDatasetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Dataset/CreateDataset",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DATASET_DELETE_DATASET: ::grpcio::Method<super::dataset::DeleteDatasetRequest, super::dataset::DeleteDatasetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Dataset/DeleteDataset",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct DatasetClient {
    client: ::grpcio::Client,
}

impl DatasetClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        DatasetClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn create_dataset_opt(&self, req: &super::dataset::CreateDatasetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::dataset::CreateDatasetResponse> {
        self.client.unary_call(&METHOD_DATASET_CREATE_DATASET, req, opt)
    }

    pub fn create_dataset(&self, req: &super::dataset::CreateDatasetRequest) -> ::grpcio::Result<super::dataset::CreateDatasetResponse> {
        self.create_dataset_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_dataset_async_opt(&self, req: &super::dataset::CreateDatasetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::dataset::CreateDatasetResponse>> {
        self.client.unary_call_async(&METHOD_DATASET_CREATE_DATASET, req, opt)
    }

    pub fn create_dataset_async(&self, req: &super::dataset::CreateDatasetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::dataset::CreateDatasetResponse>> {
        self.create_dataset_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_dataset_opt(&self, req: &super::dataset::DeleteDatasetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::dataset::DeleteDatasetResponse> {
        self.client.unary_call(&METHOD_DATASET_DELETE_DATASET, req, opt)
    }

    pub fn delete_dataset(&self, req: &super::dataset::DeleteDatasetRequest) -> ::grpcio::Result<super::dataset::DeleteDatasetResponse> {
        self.delete_dataset_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_dataset_async_opt(&self, req: &super::dataset::DeleteDatasetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::dataset::DeleteDatasetResponse>> {
        self.client.unary_call_async(&METHOD_DATASET_DELETE_DATASET, req, opt)
    }

    pub fn delete_dataset_async(&self, req: &super::dataset::DeleteDatasetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::dataset::DeleteDatasetResponse>> {
        self.delete_dataset_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Dataset {
    fn create_dataset(&mut self, ctx: ::grpcio::RpcContext, req: super::dataset::CreateDatasetRequest, sink: ::grpcio::UnarySink<super::dataset::CreateDatasetResponse>);
    fn delete_dataset(&mut self, ctx: ::grpcio::RpcContext, req: super::dataset::DeleteDatasetRequest, sink: ::grpcio::UnarySink<super::dataset::DeleteDatasetResponse>);
}

pub fn create_dataset<S: Dataset + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DATASET_CREATE_DATASET, move |ctx, req, resp| {
        instance.create_dataset(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_DATASET_DELETE_DATASET, move |ctx, req, resp| {
        instance.delete_dataset(ctx, req, resp)
    });
    builder.build()
}
