use std::sync::OnceLock;

use utils::env::get_env;

/// Generates a new [`Config`] from the environment.
///
/// This stores the [`Config`] in a [`OnceLock`] so that
/// it is only generated once. This allows you to call
/// this function each time you need data from the [`Config`]
/// without having to worry about performance.
pub fn web_config() -> &'static Config {
	static ENVIRONMENT: OnceLock<Config> = OnceLock::new();

	ENVIRONMENT.get_or_init(|| {
		Config::from_env().unwrap_or_else(|ex| panic!("Couldn't load environment: {}", ex))
	})
}

/// Configuratio for the web server.
#[allow(non_snake_case)]
pub struct Config {
	pub PUBLIC_KEY: String,
	pub PRIVATE_KEY: String,
	pub API_ORIGIN: String,
	// TODO: pub API_ORIGIN: Vec<HeaderValue>,
}

impl Config {
	/// Generates a new [`Config`] from the environment.
	///
	/// Returns an error if any of the environment
	/// variables are missing.
	pub fn from_env() -> utils::env::Result<Config> {
		Ok(Config {
			PUBLIC_KEY: get_env("PUBLIC_KEY")?,
			PRIVATE_KEY: get_env("PRIVATE_KEY")?,
			API_ORIGIN: get_env("API_ORIGIN")?,
		})
	}
}
