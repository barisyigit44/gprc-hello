use tonic::{transport::Server, Request, Response, Status};
use hello::say_server::{Say, SayServer};
use hello::{SayResponse, SayRequest};


pub mod hello {
    tonic::include_proto!("hello");
}

// defining a struct for our service
#[derive(Default)]
pub struct MySay {}

// implementing rpc for service defined in .proto

#[tonic::async_trait]
impl Say for MySay {
// our rpc impelemented as function
    async fn send(&self,request:Request<SayRequest>)->Result<Response<SayResponse>,Status>{

        println!("Got a request: {:?}", request);
        
        Ok(Response::new(SayResponse{
             message:format!("hello friend {}",request.get_ref().name),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
// defining address for our service
    let addr = "[::1]:50051".parse()?;
// creating a service
    let say = MySay::default();
    println!("Server listening on {}", addr);
// adding our service to our server.
    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}