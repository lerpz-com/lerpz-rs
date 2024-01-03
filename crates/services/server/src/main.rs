use grpc::{
	hello_server::{Hello, HelloServer},
	HelloRequest, HelloResponse,
};
use tonic::{metadata::MetadataValue, transport::Server};

#[derive(Default, Clone)]
pub struct HelloService;

#[tonic::async_trait]
impl Hello for HelloService {
	async fn say_hello(
		&self,
		request: tonic::Request<HelloRequest>,
	) -> Result<tonic::Response<HelloResponse>, tonic::Status> {
		let reply = HelloResponse {
			message: format!("Hello {}!", request.into_inner().name).into(),
		};

		Ok(tonic::Response::new(reply))
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = "[::1]:50051".parse().unwrap();

	let hello_service = HelloServer::with_interceptor(HelloService::default(), check_auth);

	Server::builder()
		.add_service(hello_service)
		.serve(addr)
		.await?;

	Ok(())
}

fn check_auth(req: tonic::Request<()>) -> tonic::Result<tonic::Request<()>, tonic::Status> {
	let valid_token: MetadataValue<_> = "Bearer some-auth-token".parse().unwrap();

	let token = match req.metadata().get("authorization") {
		Some(token) => token,
		_ => return Err(tonic::Status::unauthenticated("No valid auth token")),
	};

	if token == valid_token {
		Ok(req)
	} else {
		Err(tonic::Status::unauthenticated("Invalid auth token"))
	}
}
