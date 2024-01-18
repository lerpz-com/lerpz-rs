use std::net::{Ipv4Addr, SocketAddr};

use rpc::{
	account_server::{Account, AccountServer},
	AccountInfoResponse, AccountLookupInfo,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct AccountService {}

#[tonic::async_trait]
impl Account for AccountService {
	async fn get_account(
		&self,
		request: Request<AccountLookupInfo>,
	) -> Result<Response<AccountInfoResponse>, Status> {
		let reply = AccountInfoResponse {
			name: "test".to_string(),
			email: format!("Hello {}!", request.into_inner().email),
		};

		Ok(Response::new(reply))
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
	health_reporter
		.set_serving::<AccountServer<AccountService>>()
		.await;

	let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8081));
	let greeter = AccountService::default();

	println!("HealthServer + GreeterServer listening on {}", addr);

	Server::builder()
		.add_service(health_service)
		.add_service(AccountServer::new(greeter))
		.serve(addr)
		.await?;

	Ok(())
}
