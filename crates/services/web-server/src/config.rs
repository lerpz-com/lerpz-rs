use std::sync::OnceLock;

use utils::env::get_env;

pub fn web_config() -> &'static Config {
	static ENVIRONMENT: OnceLock<Config> = OnceLock::new();

	ENVIRONMENT.get_or_init(|| {
		Config::from_env().unwrap_or_else(|ex| panic!("Couldn't load environment: {}", ex))
	})
}

#[allow(non_snake_case)]
pub struct Config {
	pub PUBLIC_KEY: String,
	pub PRIVATE_KEY: String,
	pub API_ORIGIN: String,
	// TODO: pub API_ORIGIN: Vec<HeaderValue>,
}

impl Config {
	pub fn from_env() -> utils::env::Result<Config> {
		Ok(Config {
			PUBLIC_KEY: get_env("PUBLIC_KEY")?,
			PRIVATE_KEY: get_env("PRIVATE_KEY")?,
			API_ORIGIN: get_env("API_ORIGIN")?,
		})
	}
}
