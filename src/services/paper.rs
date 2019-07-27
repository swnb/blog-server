use crate::models;
use actix_web::{web, HttpResponse, Responder, Route};
use serde::Deserialize;
use serde_json;

// reader paper info list,each paper limit 5 row most
const PAGE_AMOUNT: u64 = 5;
fn read_paper_info(path: web::Path<(u64)>) -> impl Responder {
	let offset = *path - 1;
	let result = models::query_papers(PAGE_AMOUNT, offset * PAGE_AMOUNT).unwrap(); // FIXME: rm this unwrap

	serde_json::to_string(&result).unwrap_or_else(|_| String::from("server error"))
}

// reader paper content by paper hash id
fn read_paper_content(path: web::Path<String>) -> impl Responder {
	// copy string
	let paper_hash: &str = &*path;
	models::query_paper_content(paper_hash).unwrap_or_else(|_| String::from("server error"))
}

#[derive(Deserialize)]
struct PaperJsonParam {
	title: String,
	content: String,
	author: String,
	tags: Vec<String>,
}

// post new paper
fn post_new_paper(paper: web::Json<PaperJsonParam>) -> impl Responder {
	let PaperJsonParam {
		title: param_title,
		content: param_content,
		author: param_author,
		tags: param_tags,
	} = &*paper;

	let result = models::post_new_paper(param_title, param_content, param_author, param_tags);

	result
		.map(|_| String::from("Ok"))
		.unwrap_or_else(|_| String::from("post new paper fail ; see log file"))
}

fn alive_check(path: web::Path<String>) -> HttpResponse {
	let phrase: String = path[..100].to_owned();
	let mut response = HttpResponse::Ok();
	response.body(phrase)
}

pub fn routes<'a>() -> Vec<(&'a str, Route)> {
	vec![
		("/check/{phrase}", web::get().to(alive_check)),
		(
			"/get/paper/content/{paper_hash}",
			web::get().to(read_paper_content),
		),
		("/get/paper/infos/{page}", web::get().to(read_paper_info)),
		("/post/paper/", web::post().to(post_new_paper)),
	]
}
