#![allow(dead_code)]
#![allow(unused_variables)]

use std::net::Ipv4Addr;

use api_doc::ApiV1Doc;
use axum::{
	http::{HeaderValue, Method},
	Router,
};
use config::web_config;
use rpc::account_client::AccountClient;
use tonic::transport::Channel;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api_doc;
mod config;
mod controllers;
mod error;
mod middleware;

#[derive(Clone)]
pub struct AppState {
	pub account_conn: AccountClient<Channel>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::from_default_env())
		.init();

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

	let service = ServiceBuilder::new()
		.layer(CookieManagerLayer::new())
		.layer(cors);

	let app_state = AppState {
		account_conn: AccountClient::connect(web_config().ACCOUNT_SERVICE_URL.clone())
			.await
			.unwrap(),
	};

	let app = Router::new()
		.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiV1Doc::openapi()))
		.nest("/api/v1/auth", controllers::auth::routes().await)
		.layer(service)
		.with_state(app_state);

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
