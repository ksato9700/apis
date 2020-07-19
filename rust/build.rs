extern crate protoc_grpcio;

fn main() {
    let proto_root = "src";
    protoc_grpcio::compile_grpc_protos(&["v1/helloworld.proto"], &[proto_root], &proto_root, None)
        .expect("Failed to compile gRPC definitions!");
}
