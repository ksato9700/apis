extern crate prost_build;
extern crate prost_twirp;

fn main() {
    let mut conf = prost_build::Config::new();
    conf.service_generator(Box::new(prost_twirp::TwirpServiceGenerator::new()));
    conf.compile_protos(
        &["src/v1/helloworld.proto", "src/v1/blood_type.proto"],
        // &["src/v1/service.proto"],
        &["src/v1"],
    )
    .unwrap();
}
