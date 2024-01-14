use jsonwebtoken::{DecodingKey, TokenData, Validation};

use super::claims::{Claims, JwtAudience, JwtIssuer};

pub struct JwtDecoder {
	validation: Validation,
	token: String,
}

impl JwtDecoder {
	pub fn new(token: impl Into<String>) -> Self {
		Self {
			validation: Validation::default(),
			token: token.into(),
		}
	}

	pub fn alg(mut self, alg: jsonwebtoken::Algorithm) -> JwtDecoder {
		self.validation.algorithms = vec![alg];
		self
	}

	pub fn validate_nbf(mut self, value: bool) -> JwtDecoder {
		self.validation.validate_nbf = value;
		self
	}

	pub fn validate_aud(mut self, aud: &[JwtAudience]) -> JwtDecoder {
		self.validation.validate_aud = true;
		self.validation.set_audience(aud);
		self
	}

	pub fn validate_iss(mut self, iss: &[JwtIssuer]) -> JwtDecoder {
		self.validation.set_issuer(iss);
		self
	}

	pub fn decode(
		self,
		decoding_key: &DecodingKey,
	) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
		let token = &self.token;
		let validation = &self.validation;

		jsonwebtoken::decode::<Claims>(&token, decoding_key, &validation)
	}
}
