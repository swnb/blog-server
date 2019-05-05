#[macro_use]
extern crate diesel;

mod markdown_parser;
mod models;
mod paper;

use actix_web::{server, App};

fn main() {
	server::new(|| {
		let app = App::new();
		paper::handler(app)
	})
	.bind("127.0.0.1:9999")
	.expect("bind port 9999")
	.run();
}
