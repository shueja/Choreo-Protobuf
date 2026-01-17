use tokio::time::Instant;

use proto_rs::{entity::SwerveSample, service::{choreo_service_client::ChoreoServiceClient, commands::EchoSwerveSampleRequest}};

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

    let request = tonic::Request::new(EchoSwerveSampleRequest{
        sample: Some(SwerveSample::default())
    });

    let response = timeit(|| client.echo_swerve_sample(request)).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}