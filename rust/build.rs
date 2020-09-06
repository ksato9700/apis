extern crate protoc_grpcio;

fn main() {
    protoc_grpcio::compile_grpc_protos(
        &["helloworld.proto", "blood_type.proto"],
        &["src/v1"],
        &"src",
        None,
    )
    .expect("Failed to compile gRPC definitions!");
}
