use std::net::Ipv4Addr;

use axum::{http::Method, Router};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

pub mod error;
pub mod middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = std::net::SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
	let listener = tokio::net::TcpListener::bind(addr).await?;

	let cors = CorsLayer::new()
		.allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT])
		// TODO: change this to the actual origin
		.allow_origin(Any);

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
		tokio::signal::unix::signal(signal::unix::SignalKind::terminate())
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
