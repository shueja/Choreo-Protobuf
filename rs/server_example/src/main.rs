use proto_rs::{
    entity::{
        DriveType, Expr, GenerationOutput, SwerveTrajectory, TrajectoryFile, ValidGenerationOutput, ValidProjectFile, ValidTrajectoryFile, parameters::{
           ExprParameters, ValidExprParameters, robotconfig::{ValidExprBumper, ValidExprModule, ValidExprRobotConfig},
        }
    },
    service::{
        choreo_service_server::{ChoreoService, ChoreoServiceServer},
        commands::{
            EchoSwerveSampleRequest, EchoSwerveSampleResponse, GenerateRequest, GenerateResponse, GetDefaultTrajectoryResponse, ValidGenerateRequest, ValidGenerateResponse, ValidGetDefaultTrajectoryResponse
        },
    }, validate::{response, validate},
};
use std::{time::Duration, vec};
use tonic::{Request, Response, Status, transport::Server};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer, ExposeHeaders};
fn get_default_project_file() -> ValidProjectFile { ValidProjectFile {
            name: "New Project".to_string(),
            drive_type: DriveType::Swerve.into(),
            // variables: Variables {
            //     expressions: BTreeMap::new(),
            //     poses: BTreeMap::new(),
            // },
            config: ValidExprRobotConfig {
                gearing: Expr::new("6.5", 6.5),
                radius: Expr::new("2 in", 0.0508),
                vmax: Expr::new("6000.0 RPM", (6000.0 / 60.0) * std::f64::consts::TAU),
                tmax: Expr::new("1.2 N*m", 1.2),
                front_left: ValidExprModule {
                    x: Expr::new("11 in", 0.2794),
                    y: Expr::new("11 in", 0.2794),
                },
                front_right: ValidExprModule {
                    x: Expr::new("11 in", 0.2794),
                    y: Expr::new("-11 in", -0.2794),
                },
                back_left: ValidExprModule {
                    x: Expr::new("-11 in", -0.2794),
                    y: Expr::new("11 in", 0.2794),
                },
                back_right: ValidExprModule {
                    x: Expr::new("-11 in", -0.2794),
                    y: Expr::new("-11 in", -0.2794),                    
                },
                mass: Expr::new("150 lbs", 68.038_855_5),
                inertia: Expr::new("6 kg m^2", 6.0),
                cof: Expr::new("1.5", 1.5),
                bumper: ValidExprBumper {
                    front: Expr::new("16 in", 0.4064),
                    left: Expr::new("16 in", 0.4064),
                    back: Expr::new("16 in", 0.4064),
                    right: Expr::new("16 in", 0.4064),
                },
                differential_track_width: Expr::new("22 in", 0.2794 * 2.0),
            },
            // generation_features: Vec::new(),
            // codegen: CodeGenConfig {
            //     root: None,
            //     gen_vars: true,
            //     gen_traj_data: true,
            //     use_choreo_lib: true,
            // },
        }}
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
        let params = ValidExprParameters {
                target_dt: Expr {
                    expr: "".to_string(),
                    value: 0.05,
                },
                waypoints: vec![],
                constraints: vec![],
            };
        let trajectory_file = ValidTrajectoryFile {
            name: "NewPath".to_string(),
            params,
            snapshot: None,
            trajectory: ValidGenerationOutput {
                splits: vec![],
                waypoints: vec![],
                config: None,
                trajectory: proto_rs::entity::generation_output::Trajectory::Swerve(
                    SwerveTrajectory { samples: vec![] },
                ),
            },
        };
        response(ValidGetDefaultTrajectoryResponse{
            trajectory: trajectory_file
        })
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
