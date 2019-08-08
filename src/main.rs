#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

mod markdown_parser;
mod models;
mod server;
mod services;
mod utils;

use log::info;
use utils::config::{parse_config, Config};

type StdError = Box<dyn std::error::Error>;

fn main() {
	utils::env::set_env();
	let Config { addr } = parse_config().expect("parse config variable fail");
	info!("server is running at {}", addr);
	server::setup_server(addr).expect("can't set up server");
}
