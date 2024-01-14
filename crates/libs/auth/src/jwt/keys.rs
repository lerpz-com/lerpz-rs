use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct JwtKeys {
	pub encoding: EncodingKey,
	pub decoding: DecodingKey,
}

impl From<&str> for JwtKeys {
	fn from(key: &str) -> Self {
		let encoding = key.as_bytes();
		let decoding = key.as_bytes();

		Self {
			encoding: EncodingKey::from_secret(encoding),
			decoding: DecodingKey::from_secret(decoding),
		}
	}
}

impl From<String> for JwtKeys {
	fn from(key: String) -> Self {
		let encoding = key.as_bytes();
		let decoding = key.as_bytes();

		Self {
			encoding: EncodingKey::from_secret(encoding),
			decoding: DecodingKey::from_secret(decoding),
		}
	}
}

impl JwtKeys {
	pub fn from_ed_pem(private_key: String, public_key: String) -> Self {
		let encoding = private_key.as_bytes();
		let decoding = public_key.as_bytes();

		Self {
			encoding: EncodingKey::from_ed_pem(encoding).unwrap(),
			decoding: DecodingKey::from_ed_pem(decoding).unwrap(),
		}
	}
}
