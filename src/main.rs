#[macro_use]
extern crate diesel;

mod markdown_parser;
mod models;
mod paper;
mod utils;

use actix_web::{server, App};

fn main() {
	utils::env::set_env();
	server::new(|| {
		let app = App::new();
		paper::handler(app)
	})
	.bind("127.0.0.1:9999")
	.expect("bind port 9999")
	.run();
}
