pub mod connect;
pub mod schema;
pub mod table;

use super::markdown_parser;
use connect::get_connection;
use diesel::prelude::*;
use schema::{paper_tags::dsl as paper_tags_dsl, papers::dsl as papers_dsl};
use table::{PaperContent, PaperInfo};

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
pub fn query_paper_infos(page_amount: i64, offset: i64) -> Result<Vec<PaperInfo>, Error> {
	let connection = &*get_connection().get().expect("can't set connection");
	use papers_dsl::{author, create_time, hash, last_change_time, papers, title};
	papers
		.select((title, author, last_change_time, create_time, hash))
		.limit(page_amount)
		.offset(page_amount * offset)
		.load::<PaperInfo>(connection)
		.map_err(|_| Error::Database(String::from("database error")))
}

pub fn query_paper_content(paper_hash: &str) -> Result<String, Error> {
	use papers_dsl::{content, hash, papers};
	let connection = &*get_connection().get().expect("can't set connection");
	papers
		.select((hash, content))
		.filter(hash.eq(paper_hash))
		.limit(1)
		.load::<PaperContent>(connection)
		.map_err(|_| Error::Database(String::from("database error")))
		.and_then(|result| {
			result
				.get(0)
				.ok_or(Error::Query(String::from("query result nothing")))
				.map(|row| markdown_parser::parse_markdown2html_json_struct(&row.content))
		})
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
	let connection = &*get_connection().get().expect("can't set connection");
	use papers_dsl::{author, content, create_time, hash, papers, title};
	diesel::insert_into(papers)
		.values((
			title.eq(param_title),
			content.eq(param_content),
			author.eq(param_author),
			create_time.eq(param_create_time),
			hash.eq(hash_id),
		))
		.execute(connection)
		.unwrap();

	use paper_tags_dsl::{id, paper_tags, tag};
	param_tags.iter().for_each(|param_tag| {
		diesel::insert_into(paper_tags)
			.values((tag.eq(param_tag), id.eq(hash_id)))
			.execute(connection)
			.unwrap();
	});
	Ok(1)
}

// query paper tags
pub fn get_paper_tags(param_id: &str) -> Result<Vec<String>, Error> {
	let connection = &*get_connection().get().expect("can't set connection");
	use paper_tags_dsl::{id, paper_tags, tag};

	paper_tags
		.select(tag)
		.filter(id.eq(param_id))
		.load::<String>(connection)
		.map_err(|_| Error::Query(String::from("query error")))
}

// get relatad paper by tag
pub fn get_relatad_paper_by_tag(param_tag: &str) -> Result<Vec<String>, Error> {
	let connection = &*get_connection().get().expect("can't set connection");
	use paper_tags_dsl::{id, paper_tags, tag};

	paper_tags
		.select(id)
		.filter(tag.eq(param_tag))
		.load::<String>(connection)
		.map_err(|_| Error::Query(String::from("query error")))
}
