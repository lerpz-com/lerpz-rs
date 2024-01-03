use grpc::{hello_client::HelloClient, HelloRequest};
use tonic::{metadata::MetadataValue, transport::Channel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let channel = Channel::from_static("http://[::1]:50051").connect().await?;

	let token: MetadataValue<_> = "Bearer some-auth-token".parse()?;

	let mut client = HelloClient::with_interceptor(channel, move |mut req: tonic::Request<()>| {
		req.metadata_mut().insert("authorization", token.clone());
		Ok(req)
	});

	let request = tonic::Request::new(HelloRequest {
		name: "Kanerix".into(),
	});

	let response = client.say_hello(request).await?;

	println!("RESPONSE={:?}", response);

	Ok(())
}
