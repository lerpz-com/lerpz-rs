use auth::jwt::keys::JwtKeys;

pub struct Ctx {
	pub keys: JwtKeys,
}

impl Ctx {
	pub fn new(keys: JwtKeys) -> Self {
		Self { keys }
	}
}
