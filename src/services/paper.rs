use super::response::Response;
use crate::models;

use actix_web::{cookie::Cookie, web, HttpMessage, HttpRequest, HttpResponse, Route};
use models::error::Error;
use serde::Deserialize;
use std::{collections::HashSet, sync::RwLock};

// reader paper info list,each paper limit 5 row most
const PAGE_AMOUNT: u64 = 5;
fn read_paper_info_list(path: web::Path<(u64)>) -> HttpResponse {
	let index = *path - 1;
	let offset = index * PAGE_AMOUNT;

	let result = models::query_papers(PAGE_AMOUNT, offset);
	match result {
		Ok(list) => Response::success(list),
		Err(Error::DataBaseError(reason)) => {
			println!("reason {}", reason); // TODO add log
			Response::server_error()
		}
		Err(Error::NotFound) => Response::not_found(),
		_ => Response::bad_request(),
	}
}

// reader paper content by paper id
fn read_paper_content(path: web::Path<String>) -> HttpResponse {
	// copy string
	let paper_hash: &str = &*path;
	let result = models::query_paper_content(paper_hash);
	match result {
		Ok(result) => HttpResponse::Ok().body(result), // TODO add better log and response
		Err(Error::DataBaseError(reason)) => {
			println!("reason {}", reason); // TODO add log
			Response::server_error()
		}
		Err(Error::NotFound) => Response::not_found(),
		_ => HttpResponse::BadRequest().finish(),
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
		return Response::not_authentication();
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
		Ok(_) => Response::success(""),
		Err(error) => match error {
			Error::DataBaseError(reason) => {
				println!("reason {}", reason); // TODO add log
				Response::server_error()
			}
			_ => Response::bad_request(),
		},
	}
}

// update paper use paper title
fn update_paper(
	req: HttpRequest,
	data: web::Data<AppState>,
	body: web::Json<PaperJsonParam>,
) -> HttpResponse {
	if !is_authority(&req, &data.token_set.read().unwrap()) {
		return Response::not_authentication();
	}

	let PaperJsonParam {
		title,
		author,
		content,
		tags,
	} = &*body;
	let result = models::update_paper(title, author, content, tags);
	match result {
		Ok(_) => Response::success(""),
		Err(error) => match error {
			Error::DataBaseError(reason) => {
				println!("reason {}", reason); // TODO add log
				Response::server_error()
			}
			_ => Response::bad_request(),
		},
	}
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
		return Response::not_authentication();
	}
	let PaperTagsParam { title, tags } = &*body;
	match models::add_tags(title, tags) {
		Ok(_) => Response::success(""),
		Err(error) => match error {
			Error::DataBaseError(reason) => {
				println!("reason {}", reason); // TODO add log
				Response::server_error()
			}
			_ => Response::bad_request(),
		},
	}
}

fn alive_check() -> HttpResponse {
	Response::success("success init")
}

pub fn routes<'a>() -> Vec<(&'a str, Route)> {
	use web::{get, post, put};
	vec![
		("/check", get().to(alive_check)),
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
