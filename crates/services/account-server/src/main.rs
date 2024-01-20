use std::net::{Ipv4Addr, SocketAddr};

use account::AccountService;
use rpc::account_server::AccountServer;
use tonic::transport::Server;

mod account;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8081));

	let account_service = AccountServer::new(AccountService::default());

	Server::builder()
		.add_service(account_service)
		.serve(addr)
		.await?;

	Ok(())
}
