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
		claims::{JwtAudience, JwtIssuer, JwtUser},
		decode::JwtDecoder,
		encode::JwtEncoder,
		generate_refresh_token,
		keys::JwtKeys,
	};

	#[test]
	fn encode_and_decode() {
		let user = JwtUser {
			id: uuid::Uuid::new_v4(),
			username: "test".to_string(),
			email: "test@test.com".to_string(),
			role: UserRole::ADMIN,
		};

		let keys = JwtKeys::from("secret");

		let token = JwtEncoder::new(user)
			.aud(JwtAudience::MainWebsite)
			.iss(JwtIssuer::API)
			.encode(&keys.encoding)
			.unwrap();
		let token_data = JwtDecoder::new(token)
			.validate_nbf(true)
			.validate_aud(&[JwtAudience::MainWebsite])
			.validate_iss(&[JwtIssuer::API])
			.decode(&keys.decoding)
			.unwrap();

		assert_eq!(token_data.claims.aud, JwtAudience::MainWebsite);
		assert_eq!(token_data.claims.iss, JwtIssuer::API);
	}

	#[test]
	pub fn invalid_iss_and_aud() {
		let user = JwtUser {
			id: uuid::Uuid::new_v4(),
			username: "test".to_string(),
			email: "test@test.com".to_string(),
			role: UserRole::ADMIN,
		};

		let keys = JwtKeys::from("secret");

		let token = JwtEncoder::new(user)
			.aud(JwtAudience::MainWebsite)
			.iss(JwtIssuer::API)
			.encode(&keys.encoding)
			.unwrap();
		let token_data = JwtDecoder::new(token)
			.validate_nbf(true)
			.validate_aud(&[JwtAudience::Account])
			.validate_iss(&[JwtIssuer::API])
			.decode(&keys.decoding);

		assert!(token_data.is_err());
	}

	#[test]
	fn generate_refresh_token_length() {
		let token = generate_refresh_token();
		assert_eq!(token.len(), 32);
	}
}
