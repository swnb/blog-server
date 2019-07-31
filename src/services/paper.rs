use super::response::*;
use crate::models;
use actix_web::{cookie::Cookie, web, HttpMessage, HttpRequest, HttpResponse, Route};
use serde::Deserialize;
use std::{collections::HashSet, sync::RwLock};

// reader paper info list,each paper limit 5 row most
const PAGE_AMOUNT: u64 = 5;
fn read_paper_info_list(path: web::Path<(u64)>) -> HttpResponse {
	let offset = *path - 1;
	match models::query_papers(PAGE_AMOUNT, offset * PAGE_AMOUNT) {
		Ok(list) => HttpResponse::Ok().json(CustomResponse::success(list)), // TODO add log
		Err(_) => HttpResponse::NotFound().json(CustomResponse::not_found()),
	}
}

// reader paper content by paper id
fn read_paper_content(path: web::Path<String>) -> HttpResponse {
	// copy string
	let paper_hash: &str = &*path;
	match models::query_paper_content(paper_hash) {
		Ok(result) => HttpResponse::Ok().body(result), // TODO add better log and response
		Err(_) => HttpResponse::NotFound().json(CustomResponse::not_found()),
	}
}

// hashset store tokens
type TokenSet = HashSet<String>;

// actix web app state
pub struct AppState {
	pub token_set: RwLock<TokenSet>,
}

// check cookie is authority or not
fn is_authority(req: &HttpRequest, token_set: &TokenSet) -> bool {
	req.cookie("token")
		.and_then(|token| token_set.get(token.value()).map(|_| ()))
		.is_some()
}

// set cookie token
fn authority_response(token_set: &mut TokenSet) -> HttpResponse {
	let cookie = loop {
		// generate uuid
		let token = uuid::Uuid::new_v4();
		if token_set.insert(token.to_string()) {
			let mut cookie = Cookie::new("token", token.to_string());
			cookie.set_max_age(chrono::Duration::hours(24));
			cookie.set_http_only(true);
			break cookie;
		}
	};
	HttpResponse::Ok().cookie(cookie).finish()
}

// get token and store token use uuid
// TODO add passwd and user name
fn login(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
	if is_authority(&req, &data.token_set.read().unwrap()) {
		HttpResponse::Ok().finish()
	} else {
		authority_response(&mut data.token_set.write().unwrap())
	}
}

#[derive(Deserialize)]
struct PaperJsonParam {
	title: String,
	content: String,
	author: String,
	tags: Vec<String>,
}

// post new paper
fn post_new_paper(
	req: HttpRequest,
	data: web::Data<AppState>,
	paper: web::Json<PaperJsonParam>,
) -> HttpResponse {
	if !is_authority(&req, &data.token_set.read().unwrap()) {
		return HttpResponse::Forbidden().json(CustomResponse::not_authentication());
	}

	let PaperJsonParam {
		title,
		content,
		author,
		tags,
	} = &*paper;
	println!("posting paper {}", title);
	let result = models::post_new_paper(title, author, content, tags);
	match result {
		Ok(_) => HttpResponse::Ok().json(CustomResponse::success("")),
		// TODO log error
		Err(_) => HttpResponse::InternalServerError().json(CustomResponse::server_error()),
	}
}

// update paper use paper title
fn update_paper(
	req: HttpRequest,
	data: web::Data<AppState>,
	body: web::Json<PaperJsonParam>,
) -> HttpResponse {
	if !is_authority(&req, &data.token_set.read().unwrap()) {
		return HttpResponse::Forbidden().json(CustomResponse::not_authentication());
	}

	let PaperJsonParam {
		title,
		author,
		content,
		tags,
	} = &*body;
	models::update_paper(title, author, content, tags);
	// TODO: add error handle
	HttpResponse::Ok().json(CustomResponse::success(""))
}

// insert tags into tags column
// only append new tags use paper title
#[derive(Deserialize)]
struct PaperTagsParam {
	title: String,
	tags: Vec<String>,
}

// only append tags , not replace tags
fn put_tags(
	req: HttpRequest,
	data: web::Data<AppState>,
	body: web::Json<PaperTagsParam>,
) -> HttpResponse {
	if !is_authority(&req, &data.token_set.read().unwrap()) {
		return HttpResponse::Forbidden().json(CustomResponse::not_authentication());
	}
	let PaperTagsParam { title, tags } = &*body;
	match models::add_tags(title, tags) {
		Ok(_) => HttpResponse::Ok().json(CustomResponse::success("")),
		Err(_) => HttpResponse::InternalServerError().json(CustomResponse::server_error()),
	}
}

fn alive_check(path: web::Path<String>) -> HttpResponse {
	let phrase: String = path.to_owned();
	HttpResponse::Ok().json(CustomResponse::success(phrase))
}

pub fn routes<'a>() -> Vec<(&'a str, Route)> {
	use web::{get, post, put};
	vec![
		("/check/{phrase}", get().to(alive_check)),
		(
			"/get/paper/content/{paper_id}",
			get().to(read_paper_content),
		),
		("/get/paper/infos/{page}", get().to(read_paper_info_list)),
		("/post/paper/", post().to(post_new_paper)),
		("/update/paper/", put().to(update_paper)),
		("/put/tags/", put().to(put_tags)),
		("/login", post().to(login)),
	]
}
