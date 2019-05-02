mod markdown_parser;
mod paper;
use actix_web::{server, App};

fn main() {
	server::new(|| {
		let app = App::new();
		let app = paper::reader_paper(app);
		app
	})
	.bind("127.0.0.1:9999")
	.expect("bind port 9999")
	.run();
}
