use super::models;
use actix_web::{http::Method, App, HttpRequest, Json, Path, Responder};
use serde::Deserialize;
use serde_json;
use uuid::Uuid;

// reader paper info list,each paper limit 5 row most
const PAGE_AMOUNT: i64 = 5;
fn read_paper_info(path: Path<(i64)>) -> impl Responder {
	let offset = *path - 1;
	let result = models::query_paper_infos(PAGE_AMOUNT, offset).unwrap();

	serde_json::to_string(&result).unwrap_or_else(|_| String::from("server error"))
}

// reader paper content by paper hash id
fn read_paper_content(path: Path<String>) -> impl Responder {
	// copy string
	let paper_hash: &str = &*path;
	models::query_paper_content(paper_hash).unwrap_or_else(|_| String::from("server error"))
}

#[derive(Deserialize)]
struct PaperJsonParam {
	title: String,
	content: String,
	author: String,
	create_time: String,
	tags: Vec<String>,
}

// post new paper
fn post_new_paper(paper: Json<PaperJsonParam>) -> impl Responder {
	let PaperJsonParam {
		title: param_title,
		content: param_content,
		author: param_author,
		create_time: param_create_time,
		tags: param_tags,
	} = &*paper;

	let hash_id: String = Uuid::new_v4().to_string();

	let result = models::post_paper(
		param_title,
		param_content,
		param_author,
		param_create_time,
		param_tags,
		&hash_id,
	);

	result
		.map(|_| String::from("Ok"))
		.unwrap_or_else(|_| String::from("post new paper fail ; see log file"))
}

fn alive_check(req: &HttpRequest) -> impl Responder {
	"server success init"
}

pub fn handler(app: App<()>) -> App<()> {
	app.scope("/blog", |scope| {
		scope
			.resource("/check", |r| r.h(alive_check))
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
