use proto_rs::{entity::{ForceVector, SwerveSample}, service::{choreo_service_server::{ChoreoService, ChoreoServiceServer}, commands::{EchoSwerveSampleRequest, EchoSwerveSampleResponse}}};
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
struct ChoreoServerImpl {}
#[tonic::async_trait]
impl ChoreoService for ChoreoServerImpl {
    async fn echo_swerve_sample(
        &self,
        request: Request<EchoSwerveSampleRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<EchoSwerveSampleResponse>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);
        let request = request.into_inner();
        let _alpha = request.sample.unwrap().alpha;
        let reply = EchoSwerveSampleResponse {
            sample: request.sample
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = ChoreoServerImpl {};

    Server::builder()
        .accept_http1(true)
       // This will apply the gRPC-Web translation layer
        .layer(GrpcWebLayer::new())
        .add_service(ChoreoServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
