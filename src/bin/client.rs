
extern crate grpcio;
extern crate grpcio_proto;
#[macro_use]
extern crate log;
extern crate grpcio_example;

use std::sync::Arc;

use grpcio_example::init_log;
use grpcio::{ChannelBuilder, EnvBuilder};

use grpcio_proto::example::helloworld::HelloRequest;
use grpcio_proto::example::helloworld_grpc::GreeterClient;


fn main() {
    let _guard = init_log(None);
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::new();
    req.set_name("world aadsf".to_owned());
    let reply = client.say_hello(&req).expect("rpc");
    info!("Greeter received: {}", reply.get_message());
}
