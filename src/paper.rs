use super::markdown_parser;
use super::models;
use actix_web::{http::Method, App, HttpRequest, Path, Responder};
use diesel::prelude::*;
use serde_json;
use std::fs;
use uuid::Uuid;

// reader paper info list,each paper list 5 row most
const PAGE_AMOUNT: i64 = 5;
fn read_paper_info(path: Path<(i64)>) -> impl Responder {
	use models::{schema::papers::dsl::*, table::PaperInfo};
	let connection = models::connect();
	let offset = *path - 1;
	let result = papers
		.select((title, author, last_change_time, create_time, tags, hash))
		.limit(PAGE_AMOUNT)
		.offset(PAGE_AMOUNT * offset)
		.load::<PaperInfo>(&connection)
		.unwrap();

	match serde_json::to_string(&result) {
		Ok(result) => result,
		Err(_) => String::from("server error"),
	}
}

// reader paper content by paper id
fn read_paper_content(path: Path<String>) -> impl Responder {
	use models::{schema::papers::dsl::*, table::Paper};
	let connection = models::connect();

	// copy string
	let ref paper_hash: String = *path;
	let mut result = papers
		.filter(hash.eq(paper_hash))
		.limit(5)
		.load::<Paper>(&connection)
		.expect("some thing is happen");

	result.iter_mut().for_each(|paper: &mut Paper| {
		let text = &paper.content;
		paper.content = markdown_parser::parse_markdown2html_json_struct(text);
	});

	match serde_json::to_string(&result) {
		Ok(result) => result,
		Err(_) => String::from("server error"),
	}
}

// post new paper
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
			hash.eq(gloabal_id),
		))
		.execute(&connection);

	match result {
		Ok(_) => String::from("Ok"),
		Err(err) => format!("{:?}", err),
	}
}

pub fn handler(app: App<()>) -> App<()> {
	app.scope("/blog", |scope| {
		scope
			.resource("/get/paper/content/{paper_hash}", |r| {
				r.method(Method::GET).with(read_paper_content)
			})
			.resource("/get/paper/infos/{page}", |r| {
				r.method(Method::GET).with(read_paper_info)
			})
			.resource("/post/paper/", |r| r.f(post_paper))
	})
}
