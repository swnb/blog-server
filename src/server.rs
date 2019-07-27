use actix_web::{App, HttpServer};

pub fn setup_server(addr: std::net::SocketAddr) -> Result<(), crate::StdError> {
	HttpServer::new(|| {
		let app = App::new();
		app.service(crate::services::init())
	})
	.bind(addr)?
	.run()?;
	Ok(())
}
