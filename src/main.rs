mod markdown_parser;
use std::fs;
use actix_web::{server, App, middleware, HttpRequest, Responder};

fn greet(req: &HttpRequest) -> impl Responder {
	let text = fs::read_to_string("./example.md").unwrap();
	markdown_parser::parse_markdown2html_json_struct(&text)
}

fn main() {
	server::new(|| {
		App::new().middleware(middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
			.resource("/paper/{paper_id}", |r| {
				r.f(greet);
			})
	})
	.bind("127.0.0.1:9999")
	.expect("bind port 9999")
	.run();
}
