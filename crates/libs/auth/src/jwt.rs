use core::models::user::{User, UserRole};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

struct Keys {
	encoding: EncodingKey,
	decoding: DecodingKey,
}

pub struct JwtUtil {
	keys: Keys,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtUser {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub role: UserRole,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
	sub: String,
	iss: String,
	aud: Vec<Services>,
	exp: i64,
	iat: i64,
	pub user: JwtUser,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Services {
	#[serde(rename = "file-upload")]
	FileUpload,
}

impl From<&'_ str> for Keys {
	fn from(secret: &str) -> Self {
		let key = secret.as_bytes();

		Self {
			encoding: EncodingKey::from_secret(key),
			decoding: DecodingKey::from_secret(key),
		}
	}
}

impl JwtUtil {
	pub fn new(secret: &str) -> Self {
		Self {
			keys: Keys::from(secret),
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
		&self,
		user: impl Into<JwtUser>,
		aud: &[Services],
	) -> jsonwebtoken::errors::Result<String> {
		let header = &Header::new(Algorithm::EdDSA);
		let claims = &Claims::new(user, aud);
		let encoding_key = &self.keys.encoding;

		jsonwebtoken::encode(header, &claims, encoding_key)
	}

	pub fn verify_access_token(
		&self,
		token: &str,
		audience: &[Services],
	) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
		let mut validation = Validation::new(Algorithm::EdDSA);
		validation.validate_nbf = true;
		validation.validate_aud = true;
		validation.set_audience(audience);
		let decoding_key = &self.keys.decoding;

		jsonwebtoken::decode::<Claims>(token, decoding_key, &validation)
	}
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

impl Claims {
	pub fn new(user: impl Into<JwtUser>, audience: &[Services]) -> Self {
		Self {
			sub: Uuid::new_v4().to_string(),
			iss: "api.lerpz.com".to_string(),
			aud: audience.into(),
			exp: chrono::Utc::now().timestamp() + 60 * 15,
			iat: chrono::Utc::now().timestamp(),
			user: user.into(),
		}
	}
}

impl<'a> Into<&'a [&'a str]> for Services {
	fn into(self) -> &'a [&'a str] {
		match self {
			Self::FileUpload => &["file-upload"],
		}
	}
}

impl Display for Services {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::FileUpload => write!(f, "file-upload"),
		}
	}
}
