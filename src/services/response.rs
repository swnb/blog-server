use actix_web::HttpResponse;
use serde::Serialize;
use serde_repr::*;

// custom response with self define response code
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Code {
	Success = 0,
	Null = 100,
	ServerError = 210,
	NotAuthentication = 220,
	WrongArgument = 230,
}

#[derive(Serialize)]
pub struct Response<'a, T> {
	// 0 response success
	// 100 response not found any data
	// 210 repsonse server error
	// 220 not authentication
	// 230 request argument wrong
	code: Code,
	data: T,
	detail: &'a str,
}

impl<'a, T> Response<'a, T>
where
	T: Serialize,
{
	pub fn success(data: T) -> HttpResponse {
		HttpResponse::Ok().json(Response {
			code: Code::Success,
			data,
			detail: "",
		})
	}
}

impl<'a> Response<'a, &'a str> {
	pub fn bad_request() -> HttpResponse {
		HttpResponse::BadRequest().json(Response {
			code: Code::WrongArgument,
			data: "",
			detail: "wrong input or request",
		})
	}

	pub fn not_found() -> HttpResponse {
		HttpResponse::NotFound().json(Response {
			code: Code::Null,
			data: "",
			detail: "found nothing",
		})
	}

	pub fn server_error() -> HttpResponse {
		HttpResponse::InternalServerError().json(Response {
			code: Code::ServerError,
			data: "",
			detail: "something wrong happen",
		})
	}

	pub fn not_authentication() -> HttpResponse {
		HttpResponse::Forbidden().json(Response {
			code: Code::NotAuthentication,
			data: "",
			detail: "not authorization",
		})
	}
}
