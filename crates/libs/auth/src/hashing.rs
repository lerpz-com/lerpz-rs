pub struct PasswordHasher<'a> {
	pub salt: String,
	pub config: argon2::Config<'a>,
}

impl<'a> PasswordHasher<'a> {
	pub fn new(salt: String) -> Self {
		PasswordHasher {
			salt,
			config: argon2::Config::default(),
		}
	}
}
