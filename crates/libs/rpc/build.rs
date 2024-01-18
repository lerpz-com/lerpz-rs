fn main() -> Result<(), Box<dyn std::error::Error>> {
	tonic_build::configure()
		.server_mod_attribute("account", "#[cfg(feature = \"server\")]")
		.client_mod_attribute("account", "#[cfg(feature = \"client\")]")
		.compile(&["proto/account/account.proto"], &["proto"])?;
	Ok(())
}
