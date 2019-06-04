#[macro_use]
extern crate diesel;

mod markdown_parser;
mod models;
mod paper;
mod utils;

use actix_web::{server, App};
use std::env;

fn main() {
	// set variable
	utils::env::set_env();

	let port = match env::var("SERVER_PORT") {
		Ok(port) => port,
		Err(_) => panic!("not SERVER_PORT env var set"),
	};

	let addr = String::from("0.0.0.0:") + &port;
	println!("server is running at {}", addr);

	server::new(|| {
		let app = App::new();
		paper::handler(app)
	})
	.bind(addr)
	.expect("bind port fail")
	.run();
}
