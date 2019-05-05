pub mod schema;
pub mod table;

use diesel::prelude::*;
use std::env;

pub fn connect() -> MysqlConnection {
	match env::var("MYSQL_URL") {
		Ok(connection_url) => MysqlConnection::establish(&connection_url).unwrap_or_else(|_| {
			panic!("can't connect with database");
		}),
		Err(_) => panic!("can't get mysql connect url from env"),
	}
}
