use super::markdown_parser;
use super::models;
use actix_web::{http, middleware::DefaultHeaders, App, HttpRequest, Path, Responder};
use diesel::prelude::*;
use serde_json;
use std::fs;
use uuid::Uuid;

fn read_paper(paper_id: Path<String>) -> impl Responder {
	use models::{schema::papers::dsl::*, table::Papers};

	let connection = models::connect();
	let paper_id = match paper_id.parse::<i32>() {
		Ok(paper_id) => paper_id,
		Err(_) => return String::from("can't parser paper_id"),
	};

	let mut result = papers
		.filter(id.eq(paper_id))
		.limit(5)
		.load::<Papers>(&connection)
		.expect("some thing is happen");

	result.iter_mut().for_each(|paper: &mut Papers| {
		let text = &paper.content;
		paper.content = markdown_parser::parse_markdown2html_json_struct(text);
	});

	match serde_json::to_string(&result) {
		Ok(result) => result,
		Err(_) => String::from("server error"),
	}
}

fn post_paper(_: &HttpRequest) -> impl Responder {
	use models::schema::papers::dsl::*;
	let connection = models::connect();
	let text = fs::read_to_string("./example.md").unwrap();
	let gloabal_id: String = Uuid::new_v4().to_string();

	let result = diesel::insert_into(papers)
		.values((
			title.eq("monad"),
			content.eq(text),
			author.eq("swnb"),
			tags.eq("[swnb,rust,monad]"),
			index_hash.eq(gloabal_id),
		))
		.execute(&connection);

	match result {
		Ok(_) => String::from("Ok"),
		Err(err) => format!("{:?}", err),
	}
}

pub fn reader_paper(app: App<()>) -> App<()> {
	// cors_header
	let cros_header: DefaultHeaders =
		DefaultHeaders::new().header("Access-Control-Allow-Origin", "*");

	app.middleware(cros_header)
		.resource("/get/paper/{paper_id}", |r| {
			r.method(http::Method::GET).with(read_paper)
		})
		.resource("/post/paper/", |r| r.f(post_paper))
}
