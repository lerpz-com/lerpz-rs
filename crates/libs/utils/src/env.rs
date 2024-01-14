use std::{env, str::FromStr};

pub fn get_env(env: &'static str) -> Result<String> {
	env::var(env).map_err(|_| Error::MissingEnv(env))
}

pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
	let val = get_env(name)?;
	val.parse::<T>().map_err(|_| Error::WrongFormat(name))
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Missing environment variable: {0}")]
	MissingEnv(&'static str),
	#[error("Could't parse enviroment variable: {0}")]
	WrongFormat(&'static str),
}
