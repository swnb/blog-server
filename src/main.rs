#[macro_use]
extern crate diesel;

mod markdown_parser;
mod models;
mod paper;
mod utils;


use actix_web::{server, App};

fn main() {
	// set variable
	utils::env::set_env();

	server::new(|| {
		let app = App::new();
		paper::handler(app)
	})
	.bind("127.0.0.1:80")
	.expect("bind port 80")
	.run();
}
