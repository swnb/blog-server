use env_logger;
use std::env;

pub fn set_logger(rust_log: &str) {
	env::set_var("RUST_LOG", rust_log);
	env_logger::init();
}
