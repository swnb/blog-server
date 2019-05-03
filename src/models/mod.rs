pub mod schema;
pub mod table;

use diesel::prelude::*;

pub fn connect() -> MysqlConnection {
	let connection_url = "mysql://root:123@127.0.0.1:3243/tmp";

	MysqlConnection::establish(&connection_url).unwrap_or_else(|_| {
		panic!("can't connect with database");
	})
}
