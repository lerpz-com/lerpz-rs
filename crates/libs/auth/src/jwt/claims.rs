use core::models::user::{User, UserRole};
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
	pub sub: Uuid,
	pub exp: i64,
	pub nbf: i64,
	pub iat: i64,
	pub iss: JwtIssuer,
	pub aud: JwtAudience,
	pub user: JwtUser,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtUser {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub role: UserRole,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum JwtAudience {
	#[serde(rename = "https://lerpz.com")]
	MainWebsite,
	#[serde(rename = "https://account.lerpz.com")]
	Account,
	#[serde(rename = "https://dashboard.lerpz.com")]
	Dashboard,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum JwtIssuer {
	#[serde(rename = "https://api.lerpz.com")]
	API,
}

impl Claims {
	pub fn new(user: impl Into<JwtUser>) -> Self {
		Self {
			sub: Uuid::new_v4(),
			exp: chrono::Utc::now().timestamp() + 60 * 15,
			nbf: chrono::Utc::now().timestamp(),
			iat: chrono::Utc::now().timestamp(),
			iss: JwtIssuer::API,
			aud: JwtAudience::MainWebsite,
			user: user.into(),
		}
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

impl From<JwtUser> for Claims {
	fn from(user: JwtUser) -> Self {
		Self {
			sub: Uuid::new_v4(),
			exp: chrono::Utc::now().timestamp() + 60 * 15,
			nbf: chrono::Utc::now().timestamp(),
			iat: chrono::Utc::now().timestamp(),
			iss: JwtIssuer::API,
			aud: JwtAudience::MainWebsite,
			user,
		}
	}
}

impl Display for JwtAudience {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JwtAudience::MainWebsite => write!(f, "{}", "https://lerpz.com"),
			JwtAudience::Account => write!(f, "{}", "https://account.lerpz.com"),
			JwtAudience::Dashboard => write!(f, "{}", "https://dashboard.lerpz.com"),
		}
	}
}

impl Display for JwtIssuer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JwtIssuer::API => write!(f, "{}", "https://api.lerpz.com"),
		}
	}
}
