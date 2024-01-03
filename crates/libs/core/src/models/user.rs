use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub password: String,
	pub role: UserRole,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct RefreshToken {
	pub id: u32,
	pub token: String,
	pub user_id: Uuid,
	pub disabled_reason: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub expires_at: DateTime<Utc>,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(sqlx::Type, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
	MANAGER,
	ADMIN,
	MODERATOR,
	USER,
}
