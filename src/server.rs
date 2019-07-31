use actix_web::{middleware::Logger, App, HttpServer};

pub fn setup_server(addr: std::net::SocketAddr) -> Result<(), crate::StdError> {
	HttpServer::new(|| {
		let app = App::new();
		app.wrap(Logger::default()).service(crate::services::init())
	})
	.bind(addr)?
	.run()?;
	Ok(())
}
