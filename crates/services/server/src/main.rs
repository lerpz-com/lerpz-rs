use auth::jwt::verify_access_token;
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

	let hello_service = HelloServer::with_interceptor(HelloService::default(), check_auth);

	Server::builder()
		.add_service(hello_service)
		.serve(addr)
		.await?;

	Ok(())
}

fn check_auth(req: tonic::Request<()>) -> tonic::Result<tonic::Request<()>, tonic::Status> {
	let token_raw = match req.metadata().get("authorization") {
		Some(token) => token,
		None => return Err(tonic::Status::unauthenticated("No valid auth token")),
	};

	let token = match token_raw.to_str() {
		Ok(token) => token,
		Err(_) => {
			return Err(tonic::Status::unauthenticated(
				"Token contains invalid characters",
			))
		}
	};

	let _decoded = match verify_access_token(token) {
		Ok(decoded) => decoded,
		Err(_) => return Err(tonic::Status::unauthenticated("Invalid token")),
	};

	// TODO: Role authorization/scope

	Ok(req)
}
