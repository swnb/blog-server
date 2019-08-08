use std::{env, net::SocketAddr};

pub struct Config {
	pub addr: SocketAddr,
}

pub fn parse_config() -> Result<Config, crate::StdError> {
	let server = env::var("SERVER")?;
	let port = env::var("SERVER_PORT")?;
	let addr = format!("{}:{}", server, port).parse()?;
	Ok(Config { addr })
}
