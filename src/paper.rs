use super::markdown_parser;
use super::models;
use actix_web::{http::Method, App, Json, Path, Responder};
use diesel::prelude::*;
use serde::Deserialize;
use serde_json;
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

#[derive(Deserialize)]
struct PaperJsonParam {
	title: String,
	content: String,
	author: Option<String>,
	tags: Vec<String>,
}

// post new paper
fn post_paper(paper: Json<PaperJsonParam>) -> impl Responder {
	use models::schema::papers::dsl::*;
	let connection = models::connect();
	let PaperJsonParam {
		title: param_title,
		content: param_content,
		author: param_author,
		tags: param_tags,
	} = &*paper;
	let param_author: String = if let Some(param_author) = param_author {
		param_author.to_owned()
	} else {
		String::from("swnb")
	};
	let hash_id: String = Uuid::new_v4().to_string();

	let result = diesel::insert_into(papers)
		.values((
			title.eq(param_title),
			content.eq(param_content),
			author.eq(param_author),
			tags.eq(param_tags.join(",")),
			hash.eq(hash_id),
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
			.resource("/post/paper/", |r| r.method(Method::POST).with(post_paper))
	})
}
