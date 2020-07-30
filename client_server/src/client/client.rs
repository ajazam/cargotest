use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;

    let args: Vec<String> = env::args().collect();
    println!("args are {:?}", args);
    let addr = format!("http://{}:50051", args[1]);
    println!("addr is {:?}", addr);
    let mut client = GreeterClient::connect(addr).await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
