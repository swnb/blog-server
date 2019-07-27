#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

mod markdown_parser;
mod models;
mod server;
mod services;
mod utils;

use env_logger;
use std::env;

type StdError = Box<dyn std::error::Error>;

fn main() -> Result<(), StdError> {
	// set develop variable
	utils::env::set_env();
	env_logger::init();

	let port = env::var("SERVER_PORT")?;
	let addr = format!("0.0.0.0:{}", port);
	println!("server is running at {}", addr);

	server::setup_server(addr.parse()?)
}
