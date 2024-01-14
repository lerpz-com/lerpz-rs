use std::net::{Ipv4Addr, SocketAddrV4};

use auth::jwt::keys::JwtKeys;
use axum::{
	http::{HeaderValue, Method},
	Router,
};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

mod context;
pub mod error;

#[derive(Deserialize, Debug)]
struct Environment {
	private_key: String,
	public_key: String,
	api_origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing_subscriber::fmt::init();

	dotenv::dotenv().ok();

	let env = match envy::from_env::<Environment>() {
		Ok(env) => env,
		Err(e) => {
			tracing::error!("Failed to load environment variables: {}", e);
			std::process::exit(1);
		}
	};

	let _keys = JwtKeys::from_ed_pem(env.private_key, env.public_key);

	let addr = std::net::SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
	let listener = tokio::net::TcpListener::bind(addr).await?;

	let cors = CorsLayer::new()
		.allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT])
		.allow_origin(env.api_origin.parse::<HeaderValue>().unwrap());

	let service = ServiceBuilder::new().layer(cors);

	let app = Router::new().layer(service);

	axum::serve(listener, app.into_make_service())
		.with_graceful_shutdown(shutdown_signal())
		.await?;

	Ok(())
}

async fn shutdown_signal() {
	let ctrl_c = async {
		tokio::signal::ctrl_c()
			.await
			.expect("failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c => {
			tracing::info!("Ctrl+C received, starting graceful shutdown");
		},
		_ = terminate => {
			tracing::info!("SIGTERM received, starting graceful shutdown");
		},
	}
}
