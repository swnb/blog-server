pub mod schema;
pub mod table;

use super::markdown_parser;
use diesel::prelude::*;
use schema::papers::dsl::*;
use std::env;
use std::{thread, time};

pub fn connect() -> MysqlConnection {
	let mut connect_counter = 0;
	loop {
		let conncet_url = match env::var("MYSQL_URL") {
			Ok(connect_url) => connect_url,
			Err(_) => panic!("can't get mysql connect url"),
		};

		match MysqlConnection::establish(&conncet_url) {
			Ok(connection) => break connection,
			Err(_) => {
				// connect database with connection url in 30 min;
				if connect_counter > 30 {
					panic!("can't connect mysql database");
				}
				connect_counter += 1;
				thread::sleep(time::Duration::from_secs(1));
				continue;
			}
		}
	}
}

// define error enum
pub enum Error {
	Query(String),
	Database(String),
}

impl std::fmt::Debug for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		match self {
			Error::Query(message) => write!(f, "err query : {}", message),
			Error::Database(message) => write!(f, "err database : {}", message),
		}
	}
}

// query paper infos from database
use table::PaperInfo;
pub fn query_paper_infos(page_amount: i64, offset: i64) -> Result<Vec<PaperInfo>, Error> {
	let connection = connect();
	let result = papers
		.select((title, author, last_change_time, create_time, tags, hash))
		.limit(page_amount)
		.offset(page_amount * offset)
		.load::<PaperInfo>(&connection);
	if let Ok(result) = result {
		Ok(result)
	} else {
		Err(Error::Database(String::from("database error")))
	}
}

use table::PaperContent;
pub fn query_paper_content(paper_hash: &str) -> Result<String, Error> {
	let connection = connect();
	let result = papers
		.select((hash, content))
		.filter(hash.eq(paper_hash))
		.limit(1)
		.load::<PaperContent>(&connection);

	if let Ok(result) = result {
		if let Some(row) = result.get(0) {
			Ok(markdown_parser::parse_markdown2html_json_struct(
				&row.content,
			))
		} else {
			Err(Error::Query(String::from("query result nothing")))
		}
	} else {
		Err(Error::Database(String::from("database error")))
	}
}

// insert new paper
pub fn post_paper(
	param_title: &str,
	param_content: &str,
	param_author: &str,
	param_create_time: &str,
	param_tags: &Vec<String>,
	hash_id: &str,
) -> Result<usize, Error> {
	let connection = connect();
	diesel::insert_into(papers)
		.values((
			title.eq(param_title),
			content.eq(param_content),
			author.eq(param_author),
			create_time.eq(param_create_time),
			tags.eq(param_tags.join(",")),
			hash.eq(hash_id),
		))
		.execute(&connection)
		.unwrap();

	Ok(1)
	// .map_err(|_| Error::Database(String::from("can't get post result from database")))
}
