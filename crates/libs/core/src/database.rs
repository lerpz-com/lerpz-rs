pub enum DbError {
	InvalidUrl,
}

pub fn init_db() -> Result<sqlx::PgPool, DbError> {
	todo!()
}
