use tokio::time::Instant;

use proto_rs::{entity::{RequiredTrajectoryFile, SwerveSample, generation_output::Trajectory}, service::{choreo_service_client::ChoreoServiceClient, commands::{EchoSwerveSampleRequest, GenerateRequest}}};

async fn timeit<F: AsyncFnOnce() -> T, T>(f: F) -> T {
  let start = Instant::now();
  let result = f().await;
  let end = Instant::now();
  let duration = end.duration_since(start);
  println!("it took {} microseconds", duration.as_micros());
  result
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChoreoServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(::pbjson_types::Empty{});

    let response = timeit(|| client.get_default_trajectory(request)).await?.into_inner();
    println!("RESPONSE={:?}", response);
    let generate_request = tonic::Request::new(GenerateRequest{
        trajectory: response.trajectory.clone()
    });
    let generate_response = client.generate(generate_request).await?;
    println!("GENERATE_RESPONSE={:?}", generate_response);

    let mut broken_trajectory = response.trajectory.clone().unwrap();
    // broken_trajectory.params = None;
    // broken_trajectory.snapshot = None;
    let generate_request = tonic::Request::new(GenerateRequest{
        trajectory: Some(broken_trajectory)
    });
    let generate_response = client.generate(generate_request).await?;
    println!("FAIL_RESPONSE={:?}", generate_response);

    

    Ok(())
}