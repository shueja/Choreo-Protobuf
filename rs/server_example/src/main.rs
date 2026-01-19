use proto_rs::{
    entity::{
        ForceVector, GenerationOutput, ValidGenerationOutput, ValidTrajectoryFile,
        SwerveSample, SwerveTrajectory, TrajectoryFile, Expr,
        parameters::{
            DoubleParameters, ExprParameters, ValidDoubleParameters,
            ValidExprParameters, robotconfig::{DoubleBumper, DoubleModule, DoubleRobotConfig, ValidDoubleBumper, ValidDoubleModule, ValidDoubleRobotConfig},
        },
    },
    service::{
        choreo_service_server::{ChoreoService, ChoreoServiceServer},
        commands::{
            EchoSwerveSampleRequest, EchoSwerveSampleResponse, GenerateRequest, GenerateResponse, GetDefaultTrajectoryResponse, ValidGenerateRequest, ValidGenerateResponse, ValidGetDefaultTrajectoryResponse
        },
    }, validate::{response, validate},
};
use std::{str::FromStr, time::Duration, vec};
use tonic::{Request, Response, Status, transport::Server};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowHeaders, AllowOrigin, Any, CorsLayer, ExposeHeaders};
struct ChoreoServerImpl {}
#[tonic::async_trait]
impl ChoreoService for ChoreoServerImpl {
    async fn echo_swerve_sample(
        &self,
        request: Request<EchoSwerveSampleRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<EchoSwerveSampleResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);
        let request = request.into_inner();
        let _alpha = request.sample.unwrap().alpha;
        let reply = EchoSwerveSampleResponse {
            sample: request.sample,
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn generate(
        &self,
        request: Request<GenerateRequest>,
    ) -> Result<Response<GenerateResponse>, Status> {
        let ValidGenerateRequest { mut trajectory } = validate(request)?;
        trajectory.name="TEST123".to_string();
        trajectory.params.target_dt = Expr { value: 1.0, expr: "1.0 s".to_string() };
        // do things with the non-optioned struct
        response(ValidGenerateResponse {
            trajectory
        })
    }
    async fn get_default_trajectory(
        &self,
        _: Request<pbjson_types::Empty>,
    ) -> Result<Response<GetDefaultTrajectoryResponse>, Status> {
        let params: TrajectoryFile = TrajectoryFile {
            name: "NewPath".to_string(),
            params: Some(ExprParameters {
                target_dt: Some(Expr {
                    expr: "".to_string(),
                    value: 0.05,
                }),
                waypoints: vec![],
                constraints: vec![],
            }),
            snapshot: Some(DoubleParameters {
                target_dt: 0.05,
                waypoints: vec![],
                constraints: vec![],
            }),
            trajectory: Some(GenerationOutput {
                splits: vec![],
                waypoints: vec![],
                config: Some(DoubleRobotConfig {
                    mass: 0.0,
                    inertia: 0.0,
                    gearing: 0.0,
                    radius: 0.0,
                    vmax: 0.0,
                    tmax: 0.0,
                    cof: 0.0,
                    differential_track_width: 0.0,
                    bumper: Some(DoubleBumper {front: 0.0, left:0.0, right:0.0, back:0.0}),
                    front_left: Some(DoubleModule { x: 0.0, y: 0.0 }),
                    front_right: Some(DoubleModule { x: 0.0, y: 0.0 }),
                    back_left: Some(DoubleModule { x: 0.0, y: 0.0 }),
                    back_right: Some(DoubleModule { x: 0.0, y: 0.0 }),
                }),
                trajectory: Some(proto_rs::entity::generation_output::Trajectory::Swerve(
                    SwerveTrajectory { samples: vec![] },
                )),
            }),
        };
        Ok(Response::new(GetDefaultTrajectoryResponse{
            trajectory: Some(params) 
        }))
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
        .expose_headers(ExposeHeaders::any())
        .allow_headers(AllowHeaders::any());
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
