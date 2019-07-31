pub mod connect;
pub mod error;

use super::markdown_parser;
use connect::get_connection;
use error::Error;
use {
	diesel::prelude::*,
	serde::{Deserialize, Serialize},
	std::time::SystemTime,
};

// papers table schema
table! {
	papers (id) {
		id -> Varchar,
		title -> Varchar,
		author -> Varchar,
		content -> Text,
		create_at -> Timestamp,
		change_records -> Array<Timestamp>,
		tags  -> Array<Varchar>,
		is_draft -> Bool,
		is_del -> Bool,
	}
}

// table papers define all column struct
#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[table_name = "papers"]
pub struct Paper {
	id: String,
	title: String,
	content: String,
	author: String,
	create_at: SystemTime,
	change_records: Vec<SystemTime>,
	tags: Vec<String>,
	is_draft: bool,
	is_del: bool,
}

// define paper info struct
#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[table_name = "papers"]
pub struct PaperInfo {
	id: String,
	title: String,
	author: String,
	create_at: SystemTime,
	tags: Vec<String>,
	is_draft: bool,
	is_del: bool,
}
// query paper list with page_amount and page index
pub fn query_papers(page_amount: u64, offset: u64) -> Result<Vec<PaperInfo>, Error> {
	let connection = &*get_connection().get().expect("can't get connection");
	use self::papers::dsl::*;

	papers
		.select((id, title, author, create_at, tags, is_draft, is_del))
		.limit(page_amount as i64)
		.offset(offset as i64)
		.load::<PaperInfo>(connection)
		.map_err(Error::from)
}

// query paper content use paper id and parse content into json struct;
#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[table_name = "papers"]
pub struct PaperContent {
	id: String,
	content: String,
}

// parse paper content
// first make content html
// then parse html into json struct
fn parse_paper_content(content: &str) -> Result<String, Error> {
	Ok(markdown_parser::parse_markdown2html_json_struct(content))
}

fn parse_paper_content_from_base64(content: &str) -> Result<String, Error> {
	use error::ignore;
	let content = base64::decode(content).map_err(ignore)?;
	let content = String::from_utf8(content).map_err(ignore)?;
	parse_paper_content(&content)
}

pub fn query_paper_content(paper_id: &str) -> Result<String, Error> {
	let connection = &*get_connection().get().expect("can't get connection");
	use self::papers::dsl;
	dsl::papers
		.select((dsl::id, dsl::content))
		.filter(dsl::id.eq(paper_id))
		.limit(1)
		.first::<PaperContent>(connection)
		.map_err(Error::from)
		.and_then(|paper_content| {
			let PaperContent { ref content, .. } = paper_content;
			parse_paper_content_from_base64(content).map_err(|_| Error::ParseError)
		})
}

// insert new paper, don't need to insert id , default use uuid_v1;
// don't need create time, default use now();
// don't need record time
// don't need is_draft default true,
// don't need is_del default false,
// post new_paper_api
// post only one paper
pub fn post_new_paper(
	param_title: &str,
	param_author: &str,
	param_content: &str,
	param_tags: &[String],
) -> Result<usize, Error> {
	let connection = &*get_connection().get().expect("can't get connection");
	use self::papers::dsl::*;
	diesel::insert_into(papers)
		.values((
			title.eq(param_title),
			content.eq(param_content),
			author.eq(param_author),
			tags.eq(param_tags),
		))
		.execute(connection)
		.map_err(Error::from)
}

// format array into postgresql
fn array_to_sql(array: &[String]) -> String {
	String::from("{")
		+ &array
			.iter()
			.map(|v| format!("\"{}\"", v))
			.collect::<Vec<String>>()
			.join(",")
			.to_string()
		+ "}"
}

// update paper content without title change
// update paper with use same title, and update record timestamp
pub fn update_paper(
	param_title: &str,
	param_author: &str,
	param_content: &str,
	param_tags: &[String],
) -> Result<usize, Error> {
	let connection = &*get_connection().get().expect("can't get connection");
	let param_content = base64::encode(&param_content);
	let raw_sql = format!(
		"update papers set 
		author = '{}',
		content = '{}',
		tags = '{}',
		change_records = change_records || now()
		where title = '{}'",
		param_author,
		param_content,
		array_to_sql(param_tags),
		param_title
	);

	diesel::sql_query(raw_sql)
		.execute(connection)
		.map_err(Error::from)
}

// insert tags into papers
// append some tags use same title
pub fn add_tags(title: &str, tags: &[String]) -> Result<usize, Error> {
	let connection = &*get_connection().get().expect("can't get connection");
	let raw_sql = format!(
		"update papers set tags = tags || '{}' where title = '{}'",
		array_to_sql(&tags),
		title
	);
	diesel::sql_query(raw_sql)
		.execute(connection)
		.map_err(Error::from)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_format_array_sql() {
		assert_eq!(
			array_to_sql(
				&(b'a'..=b'c')
					.map(char::from)
					.map(|v| v.to_string())
					.collect::<Vec<String>>()
			),
			"{\"a\",\"b\",\"c\"}"
		)
	}
}
