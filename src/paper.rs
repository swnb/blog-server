use super::markdown_parser;
use super::models;
use actix_web::{http::Method, App, Json, Path, Responder};
use diesel::prelude::*;
use serde::Deserialize;
use serde_json;
use uuid::Uuid;

// reader paper info list,each paper limit 5 row most
const PAGE_AMOUNT: i64 = 5;
fn read_paper_info(path: Path<(i64)>) -> impl Responder {
	let offset = *path - 1;
	let result = models::query_paper_infos(PAGE_AMOUNT, offset).unwrap();

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
	let paper_hash: &str = &*path;
	let mut result = papers
		.filter(hash.eq(paper_hash))
		.limit(1)
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

#[derive(Deserialize)]
struct PaperJsonParam {
	title: String,
	content: String,
	author: Option<String>,
	tags: Vec<String>,
}

// post new paper
fn post_new_paper(paper: Json<PaperJsonParam>) -> impl Responder {
	let connection = models::connect();
	let PaperJsonParam {
		title: param_title,
		content: param_content,
		author: param_author,
		tags: param_tags,
	} = &*paper;
	let param_author: &str = if let Some(param_author) = param_author {
		param_author
	} else {
		"swnb"
	};
	let hash_id: String = Uuid::new_v4().to_string();

	let result = models::post_paper(
		param_title,
		param_content,
		param_author,
		param_tags,
		&hash_id,
	);

	match result {
		Ok(_) => String::from("Ok"),
		Err(_) => String::from("post new paper fail see log file"),
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
			.resource("/post/paper/", |r| {
				r.method(Method::POST).with(post_new_paper)
			})
	})
}
