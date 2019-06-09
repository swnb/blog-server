#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

mod markdown_parser;
mod models;
mod paper;
mod utils;

use actix_web::{middleware::Logger, server, App};

use env_logger;
use std::env;
fn main() {
	// set variable
	utils::env::set_env();
	env_logger::init();

	let port = env::var("SERVER_PORT").expect("no SERVER_PORT env var set");
	let addr = "0.0.0.0:".to_owned() + &port;
	println!("server is running at {}", addr);

	server::new(|| {
		let app = App::new()
			.middleware(Logger::default())
			.middleware(Logger::new("%a %{User-Agent}i"));
		paper::handler(app)
	})
	.bind(addr)
	.expect("bind port fail")
	.run();
}
