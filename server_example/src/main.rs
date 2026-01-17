use std::{str::FromStr, time::Duration};

use proto_rs::{entity::{ForceVector, SwerveSample}, service::{choreo_service_server::{ChoreoService, ChoreoServiceServer}, commands::{EchoSwerveSampleRequest, EchoSwerveSampleResponse}}};
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowHeaders, AllowOrigin, Any, CorsLayer, ExposeHeaders};
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

    const DEFAULT_MAX_AGE: Duration = Duration::from_secs(24 * 60 * 60);

            // HeaderName::from_static("grpc-message),
            // HeaderName::from_static("grpc-status-details-bin")]

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = ChoreoServerImpl {};
    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request())
        .max_age(DEFAULT_MAX_AGE)
        .expose_headers(
            ExposeHeaders::any()
        )
        .allow_headers(
            AllowHeaders::any()
        );
    Server::builder()
        .accept_http1(true)
       // This will apply the gRPC-Web translation layer
       .layer(cors_layer)
        .layer(GrpcWebLayer::new())
        
        .add_service(ChoreoServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
