pub mod schema;
pub mod table;

use super::markdown_parser;
use diesel::prelude::*;
use schema::papers::dsl::*;

use std::env;

pub fn connect() -> MysqlConnection {
	match env::var("MYSQL_URL") {
		Ok(connection_url) => MysqlConnection::establish(&connection_url).unwrap_or_else(|_| {
			panic!("can't connect with database");
		}),
		Err(_) => panic!("can't get mysql connect url from env"),
	}
}

// define error enum
pub enum Error {
	Qyery(String),
	Database(String),
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
			Err(Error::Qyery(String::from("query result nothing")))
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
	param_tags: &Vec<String>,
	hash_id: &str,
) -> Result<usize, Error> {
	let connection = connect();
	diesel::insert_into(papers)
		.values((
			title.eq(param_title),
			content.eq(param_content),
			author.eq(param_author),
			tags.eq(param_tags.join(",")),
			hash.eq(hash_id),
		))
		.execute(&connection)
		.map_err(|_| Error::Database(String::from("can't get query result from database")))
}
