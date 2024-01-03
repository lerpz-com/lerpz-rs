use grpc::{
	hello_server::{Hello, HelloServer},
	HelloRequest, HelloResponse,
};
use tonic::transport::Server;

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

	Server::builder()
		.add_service(HelloServer::new(HelloService::default()))
		.serve(addr)
		.await?;

	Ok(())
}
