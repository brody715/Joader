extern crate protoc_grpcio;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "src/proto";
    protoc_grpcio::compile_grpc_protos(&["../protos/task.proto"], &["../protos/"], &proto_root, None)
        .expect("Failed to compile grpc!");
    Ok(())
}
