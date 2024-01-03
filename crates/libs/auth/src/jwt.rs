use core::models::user::{User, UserRole};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

static KEYS: Lazy<Keys> = Lazy::new(|| {
	// NOTE: Secret is hardcoded for now, but should be loaded from a file or env variable.
	let secret = b"secret";
	let encoding = jsonwebtoken::EncodingKey::from_secret(secret);
	let decoding = jsonwebtoken::DecodingKey::from_secret(secret);
	Keys { encoding, decoding }
});

const AUD: &'static str = "lerpz.com";
const ISS: &'static str = "auth.lerpz.com";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
	pub sub: String,
	pub iss: String,
	pub aud: String,
	pub exp: usize,
	pub iat: usize,
	pub user: JwtUser,
}

struct Keys {
	encoding: EncodingKey,
	decoding: DecodingKey,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtUser {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub role: UserRole,
}

impl From<User> for JwtUser {
	fn from(user: User) -> Self {
		Self {
			id: user.id,
			username: user.username,
			email: user.email,
			role: user.role,
		}
	}
}

pub fn generate_refresh_token() -> String {
	let rng = thread_rng();
	rng.sample_iter(&Alphanumeric)
		.take(32)
		.map(char::from)
		.collect()
}

pub fn generate_access_token(user: impl Into<JwtUser>) -> jsonwebtoken::errors::Result<String> {
	let claims = Claims {
		sub: uuid::Uuid::new_v4().to_string(),
		iss: ISS.to_string(),
		aud: AUD.to_string(),
		exp: (chrono::Utc::now().timestamp() + 60 * 15) as usize,
		iat: chrono::Utc::now().timestamp() as usize,
		user: user.into(),
	};

	jsonwebtoken::encode(&Header::new(Algorithm::EdDSA), &claims, &KEYS.encoding)
}

pub fn verify_access_token(token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
	let mut validation = Validation::new(Algorithm::EdDSA);
	validation.validate_nbf = true;

	jsonwebtoken::decode::<Claims>(token, &KEYS.decoding, &validation)
}
