mod markdown_parser;
use actix_web::{server, App, HttpRequest, Responder};

fn greet(req: &HttpRequest) -> impl Responder {
	"init project"
}

fn main() {
	server::new(|| {
		App::new()
			.resource("/", |r| r.f(greet))
	})
	.bind("127.0.0.1:9999")
	.expect("bind port 9999")
	.run();
}
