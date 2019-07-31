use serde::Serialize;
use serde_repr::*;

// custom response with self define response code
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum CustomResponseCode {
	Success = 0,
	Null = 100,
	ServerError = 210,
	NotAuthentication = 220,
}

#[derive(Serialize)]
pub struct CustomResponse<'a, T> {
	// 0 response success
	// 100 response not found any data
	// 200 repsonse server error
	// 210 not authentication
	code: CustomResponseCode,
	data: T,
	detail: &'a str,
}

// TODO use different way to response data
// TODO remove server error
impl<'a, T> CustomResponse<'a, T>
where
	T: Serialize,
{
	pub fn success(data: T) -> Self {
		CustomResponse {
			code: CustomResponseCode::Success,
			data,
			detail: "",
		}
	}
}

impl<'a> CustomResponse<'a, &'a str> {
	pub fn not_found() -> Self {
		CustomResponse {
			code: CustomResponseCode::Null,
			data: "",
			detail: "found nothing",
		}
	}

	pub fn server_error() -> Self {
		CustomResponse {
			code: CustomResponseCode::ServerError,
			data: "",
			detail: "something wrong happen",
		}
	}

	pub fn not_authentication() -> Self {
		CustomResponse {
			code: CustomResponseCode::NotAuthentication,
			data: "",
			detail: "not authorization",
		}
	}
}
