use core::models::user::{User, UserRole};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

static KEYS: Lazy<Keys> = Lazy::new(|| {
	let secret = std::env::var("JWT_SECRET").expect("\"JWT_SECRET\" must be set");
	let encoding = EncodingKey::from_secret(secret.as_bytes());
	let decoding = DecodingKey::from_secret(secret.as_bytes());
	Keys { encoding, decoding }
});

struct Keys {
	encoding: EncodingKey,
	decoding: DecodingKey,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
	pub sub: String,
	pub iss: String,
	pub aud: Audience,
	pub exp: i64,
	pub iat: i64,
	pub user: JwtUser,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtUser {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub role: UserRole,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Audience {
	#[serde(rename = "file-upload")]
	FileUpload,
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

impl Display for Audience {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::FileUpload => write!(f, "file-upload"),
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

pub fn generate_access_token(
	user: impl Into<JwtUser>,
	aud: Audience,
) -> jsonwebtoken::errors::Result<String> {
	let claims = Claims {
		sub: Uuid::new_v4().to_string(),
		iss: "api.lerpz.com".to_string(),
		aud,
		exp: (chrono::Utc::now().timestamp() + 60 * 15),
		iat: chrono::Utc::now().timestamp(),
		user: user.into(),
	};

	jsonwebtoken::encode(&Header::new(Algorithm::EdDSA), &claims, &KEYS.encoding)
}

pub fn verify_access_token(token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
	let mut validation = Validation::new(Algorithm::EdDSA);
	validation.validate_nbf = true;

	jsonwebtoken::decode::<Claims>(token, &KEYS.decoding, &validation)
}
