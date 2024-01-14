use auth::jwt::JwtDecoder;
use axum::{
	async_trait,
	extract::FromRequestParts,
	http::{header, request::Parts, StatusCode},
};

struct RequireAuth;

#[async_trait]
impl<S> FromRequestParts<S> for RequireAuth
where
	S: Send + Sync,
{
	type Rejection = StatusCode;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let auth_header = parts
			.headers
			.get(header::AUTHORIZATION)
			.and_then(|value| value.to_str().ok());

		match auth_header {
			Some(auth_header) if token_is_valid(auth_header) => Ok(Self),
			_ => Err(StatusCode::UNAUTHORIZED),
		}
	}
}

fn token_is_valid(token: &str) -> bool {
	JwtDecoder::new(token).decode();
	return true;
}
