mod paper;
use actix_web::{web, Scope};

// regist all routes with actix_web scope
pub fn init() -> Scope {
	let scope = web::scope("/api/v1/blog");
	paper::routes()
		.into_iter()
		.fold(scope, |scope, (path, route)| scope.route(path, route))
}
