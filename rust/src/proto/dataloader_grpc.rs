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

const METHOD_DATA_LOADER_CREATE_DATALOADER: ::grpcio::Method<super::dataloader::CreateDataloaderRequest, super::dataloader::CreateDataloaderResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/DataLoader/CreateDataloader",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DATA_LOADER_NEXT: ::grpcio::Method<super::dataloader::NextRequest, super::dataloader::NextResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/DataLoader/Next",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DATA_LOADER_DELETE_DATALOADER: ::grpcio::Method<super::dataloader::DeleteDataloaderRequest, super::dataloader::DeleteDataloaderResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/DataLoader/DeleteDataloader",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct DataLoaderClient {
    client: ::grpcio::Client,
}

impl DataLoaderClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        DataLoaderClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn create_dataloader_opt(&self, req: &super::dataloader::CreateDataloaderRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::dataloader::CreateDataloaderResponse> {
        self.client.unary_call(&METHOD_DATA_LOADER_CREATE_DATALOADER, req, opt)
    }

    pub fn create_dataloader(&self, req: &super::dataloader::CreateDataloaderRequest) -> ::grpcio::Result<super::dataloader::CreateDataloaderResponse> {
        self.create_dataloader_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_dataloader_async_opt(&self, req: &super::dataloader::CreateDataloaderRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::dataloader::CreateDataloaderResponse>> {
        self.client.unary_call_async(&METHOD_DATA_LOADER_CREATE_DATALOADER, req, opt)
    }

    pub fn create_dataloader_async(&self, req: &super::dataloader::CreateDataloaderRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::dataloader::CreateDataloaderResponse>> {
        self.create_dataloader_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn next_opt(&self, req: &super::dataloader::NextRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::dataloader::NextResponse> {
        self.client.unary_call(&METHOD_DATA_LOADER_NEXT, req, opt)
    }

    pub fn next(&self, req: &super::dataloader::NextRequest) -> ::grpcio::Result<super::dataloader::NextResponse> {
        self.next_opt(req, ::grpcio::CallOption::default())
    }

    pub fn next_async_opt(&self, req: &super::dataloader::NextRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::dataloader::NextResponse>> {
        self.client.unary_call_async(&METHOD_DATA_LOADER_NEXT, req, opt)
    }

    pub fn next_async(&self, req: &super::dataloader::NextRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::dataloader::NextResponse>> {
        self.next_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_dataloader_opt(&self, req: &super::dataloader::DeleteDataloaderRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::dataloader::DeleteDataloaderResponse> {
        self.client.unary_call(&METHOD_DATA_LOADER_DELETE_DATALOADER, req, opt)
    }

    pub fn delete_dataloader(&self, req: &super::dataloader::DeleteDataloaderRequest) -> ::grpcio::Result<super::dataloader::DeleteDataloaderResponse> {
        self.delete_dataloader_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_dataloader_async_opt(&self, req: &super::dataloader::DeleteDataloaderRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::dataloader::DeleteDataloaderResponse>> {
        self.client.unary_call_async(&METHOD_DATA_LOADER_DELETE_DATALOADER, req, opt)
    }

    pub fn delete_dataloader_async(&self, req: &super::dataloader::DeleteDataloaderRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::dataloader::DeleteDataloaderResponse>> {
        self.delete_dataloader_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait DataLoader {
    fn create_dataloader(&mut self, ctx: ::grpcio::RpcContext, req: super::dataloader::CreateDataloaderRequest, sink: ::grpcio::UnarySink<super::dataloader::CreateDataloaderResponse>);
    fn next(&mut self, ctx: ::grpcio::RpcContext, req: super::dataloader::NextRequest, sink: ::grpcio::UnarySink<super::dataloader::NextResponse>);
    fn delete_dataloader(&mut self, ctx: ::grpcio::RpcContext, req: super::dataloader::DeleteDataloaderRequest, sink: ::grpcio::UnarySink<super::dataloader::DeleteDataloaderResponse>);
}

pub fn create_data_loader<S: DataLoader + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DATA_LOADER_CREATE_DATALOADER, move |ctx, req, resp| {
        instance.create_dataloader(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DATA_LOADER_NEXT, move |ctx, req, resp| {
        instance.next(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_DATA_LOADER_DELETE_DATALOADER, move |ctx, req, resp| {
        instance.delete_dataloader(ctx, req, resp)
    });
    builder.build()
}
