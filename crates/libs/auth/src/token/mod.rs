pub mod claims;
pub mod decode;
pub mod encode;
pub mod keys;

use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_refresh_token() -> String {
	let rng = thread_rng();
	rng.sample_iter(&Alphanumeric)
		.take(32)
		.map(char::from)
		.collect()
}

#[cfg(test)]
mod tests {
	use core::models::user::UserRole;

	use crate::token::{
		claims::{JwtAudience, JwtIssuer, TokenUser},
		decode::AuthTokenValidator,
		encode::AuthToken,
		generate_refresh_token,
		keys::JwtKeys,
	};

	#[test]
	fn encode_and_decode() {
		let user = TokenUser {
			id: uuid::Uuid::new_v4(),
			username: "test".to_string(),
			email: "test@test.com".to_string(),
			role: UserRole::ADMIN,
		};

		let keys = JwtKeys::from("secret");

		let token = AuthToken::new(user)
			.with_alg(jsonwebtoken::Algorithm::HS512)
			.with_aud(&[JwtAudience::MainWebsite])
			.with_iss(&[JwtIssuer::API])
			.encode(&keys.encoding)
			.unwrap();
		let token_data = AuthTokenValidator::new(token)
			.with_alg(jsonwebtoken::Algorithm::HS512)
			.with_aud(&[JwtAudience::MainWebsite])
			.with_iss(&[JwtIssuer::API])
			.with_nbf_validation()
			.decode(&keys.decoding)
			.unwrap();

		assert!(token_data.claims.aud.contains(&JwtAudience::MainWebsite));
		assert!(token_data.claims.iss.contains(&JwtIssuer::API));
	}

	#[test]
	pub fn invalid_iss_and_aud() {
		let user = TokenUser {
			id: uuid::Uuid::new_v4(),
			username: "test".to_string(),
			email: "test@test.com".to_string(),
			role: UserRole::ADMIN,
		};

		let keys = JwtKeys::from("secret");

		let token = AuthToken::new(user)
			.with_aud(&[JwtAudience::MainWebsite])
			.with_iss(&[JwtIssuer::API])
			.encode(&keys.encoding)
			.unwrap();
		let token_data = AuthTokenValidator::new(token)
			.with_nbf_validation()
			// This is not the same as the token's audience
			.with_aud(&[JwtAudience::Account])
			.with_iss(&[JwtIssuer::API])
			.decode(&keys.decoding);

		assert!(token_data.is_err());
	}

	#[test]
	fn refresh_token_length() {
		let token = generate_refresh_token();
		assert_eq!(token.len(), 32);
	}
}
