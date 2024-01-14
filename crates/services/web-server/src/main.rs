use std::net::Ipv4Addr;

use auth::token::keys::JwtKeys;
use axum::{
	http::{HeaderValue, Method},
	Router,
};
use config::web_config;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

mod config;
mod controllers;
mod error;
mod middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	let _keys = JwtKeys::from_ed_pem(&web_config().PUBLIC_KEY, &web_config().PRIVATE_KEY);

	let addr = std::net::SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
	let listener = tokio::net::TcpListener::bind(addr).await?;

	let cors = CorsLayer::new()
		.allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT])
		.allow_origin(
			web_config()
				.API_ORIGIN
				.split(',')
				.map(|e| e.parse().unwrap())
				.collect::<Vec<HeaderValue>>(),
		);

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
