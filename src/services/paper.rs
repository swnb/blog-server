use crate::models;
use actix_web::{cookie::Cookie, web, HttpMessage, HttpRequest, HttpResponse, Responder, Route};
use serde::Deserialize;
use serde_json;
use std::{collections::HashSet, sync::RwLock};

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

// actix web app state
pub struct AppState {
	pub token_set: RwLock<HashSet<String>>,
}

// get token and store token use uuid
// TODO add passwd and user name
fn login(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
	req.cookie("token")
		.and_then(|token| {
			let token_set = &*data.token_set.read().unwrap();
			token_set.get(token.value()).map(|_| ())
		})
		.map_or_else(
			|| {
				let cookie = loop {
					// generate uuid
					let token = uuid::Uuid::new_v4();
					if data.token_set.write().unwrap().insert(token.to_string()) {
						let mut cookie = Cookie::new("token", token.to_string());
						cookie.set_max_age(chrono::Duration::hours(24));
						cookie.set_http_only(true);
						break cookie;
					}
				};
				HttpResponse::Ok().cookie(cookie).finish()
			},
			|_| HttpResponse::Ok().finish(),
		)
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
		title,
		content,
		author,
		tags,
	} = &*paper;
	println!("posting paper {}", title);
	let result = models::post_new_paper(title, author, content, tags);
	result
		.map(|_| String::from("Ok"))
		.unwrap_or_else(|_| String::from("post new paper fail ; see log file"))
}

// update paper use paper title
fn update_paper(body: web::Json<PaperJsonParam>) {
	let PaperJsonParam {
		title,
		author,
		content,
		tags,
	} = &*body;
	models::update_paper(title, author, content, tags);
	// TODO: add error handle
}

// insert tags into tags column
// only append new tags use paper title
#[derive(Deserialize)]
struct PaperTagsParam {
	title: String,
	tags: Vec<String>,
}

fn update_tags(
	req: HttpRequest,
	data: web::Data<AppState>,
	body: web::Json<PaperTagsParam>,
) -> HttpResponse {
	req.cookie("token")
		.map_or(HttpResponse::Forbidden().finish(), |token| {
			if data.token_set.read().unwrap().get(token.value()).is_some() {
				HttpResponse::Ok().finish()
			} else {
				HttpResponse::Forbidden().finish()
			}
		})
}

fn alive_check(path: web::Path<String>) -> HttpResponse {
	let phrase: String = path.clone();
	let mut response = HttpResponse::Ok();
	response.body(phrase)
}

pub fn routes<'a>() -> Vec<(&'a str, Route)> {
	vec![
		("/check/{phrase}", web::get().to(alive_check)),
		(
			"/get/paper/content/{paper_id}",
			web::get().to(read_paper_content),
		),
		("/get/paper/infos/{page}", web::get().to(read_paper_info)),
		("/post/paper/", web::post().to(post_new_paper)),
		("/update/paper/", web::put().to(update_paper)),
		("/update/tags/", web::put().to(update_tags)),
		("/login", web::post().to(login)),
	]
}
