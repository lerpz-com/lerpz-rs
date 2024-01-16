use jsonwebtoken::{Algorithm, EncodingKey, Header};

use super::claims::{Claims, JwtAudience, JwtIssuer, JwtUser};

pub struct AuthToken {
	header: Header,
	claims: Claims,
}

impl AuthToken {
	pub fn new(user: impl Into<JwtUser>) -> AuthToken {
		Self {
			header: Header::default(),
			claims: Claims::new(user.into()),
		}
	}

	pub fn alg(mut self, alg: Algorithm) -> AuthToken {
		self.header.alg = alg;
		self
	}

	pub fn exp(mut self, exp: i64) -> AuthToken {
		self.claims.exp = exp;
		self
	}

	pub fn nbf(mut self, nbf: i64) -> AuthToken {
		self.claims.nbf = nbf;
		self
	}

	pub fn iss(mut self, iss: JwtIssuer) -> AuthToken {
		self.claims.iss = iss;
		self
	}

	pub fn aud(mut self, aud: JwtAudience) -> AuthToken {
		self.claims.aud = aud;
		self
	}

	pub fn encode(self, encoding_key: &EncodingKey) -> jsonwebtoken::errors::Result<String> {
		let header = &self.header;
		let claims = &self.claims;

		jsonwebtoken::encode(header, claims, encoding_key)
	}
}
