mod paper;
mod response;

use actix_web::{web, Scope};
use std::{collections::HashSet, sync::RwLock};

// regist all routes with actix_web scope
pub fn init() -> Scope {
	let app_state = paper::AppState {
		token_set: RwLock::new(HashSet::new()),
	};
	let scope = web::scope("/api/v1/blog").data(app_state);
	paper::routes()
		.into_iter()
		.fold(scope, |scope, (path, route)| scope.route(path, route))
}
