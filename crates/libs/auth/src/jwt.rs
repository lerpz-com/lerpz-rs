use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};

static KEYS: Lazy<Keys> = Lazy::new(|| {
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
	pub username: String,
}

struct Keys {
	encoding: EncodingKey,
	decoding: DecodingKey,
}

pub fn generate_refresh_token() -> String {
	let rng = thread_rng();
	rng.sample_iter(&Alphanumeric)
		.take(32)
		.map(char::from)
		.collect()
}

pub fn generate_access_token(username: String) -> jsonwebtoken::errors::Result<String> {
	let claims = Claims {
		sub: uuid::Uuid::new_v4().to_string(),
		iss: ISS.to_string(),
		aud: AUD.to_string(),
		exp: (chrono::Utc::now().timestamp() + 60 * 15) as usize,
		iat: chrono::Utc::now().timestamp() as usize,
		username,
	};

	jsonwebtoken::encode(&Header::new(Algorithm::EdDSA), &claims, &KEYS.encoding)
}

pub fn verify_access_token(token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
	let mut validation = Validation::new(Algorithm::EdDSA);
	validation.validate_nbf = true;

	jsonwebtoken::decode::<Claims>(token, &KEYS.decoding, &validation)
}
