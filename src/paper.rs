use super::markdown_parser;
use actix_web::{middleware::DefaultHeaders, App, HttpRequest, Responder};
use std::fs;

fn read_paper(req: &HttpRequest) -> impl Responder {
	let text = fs::read_to_string("./example.md").unwrap();
	markdown_parser::parse_markdown2html_json_struct(&text)
}

pub fn reader_paper(app: App<()>) -> App<()> {
	// cors_header
	let cros_header: DefaultHeaders =
		DefaultHeaders::new().header("Access-Control-Allow-Origin", "*");

	app.middleware(cros_header)
		.resource("/paper/{paper_id}", |r| r.f(read_paper))
}
