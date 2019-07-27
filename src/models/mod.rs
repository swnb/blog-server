pub mod connect;
use super::markdown_parser;
use connect::get_connection;
use diesel::prelude::*;
use {
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
pub fn query_papers(page_amount: u64, offset: u64) -> Result<Vec<PaperInfo>, ()> {
	let connection = &*get_connection().get().expect("can't set connection");
	use self::papers::dsl::*;

	papers
		.select((id, title, author, create_at, tags, is_draft, is_del))
		.limit(page_amount as i64)
		.offset(offset as i64)
		.load::<PaperInfo>(connection)
		.map_err(|err| println!("error when query papers {}", err))
}

// query paper content use paper id and parse content into json struct;
#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[table_name = "papers"]
pub struct PaperContent {
	id: String,
	content: String,
}
// TODO: return error result instead of ()
pub fn query_paper_content(paper_id: &str) -> Result<String, ()> {
	let connection = &*get_connection().get().expect("can't set connection");
	use self::papers::dsl::*;

	papers
		.select((id, content))
		.filter(id.eq(paper_id))
		.limit(1)
		.load::<PaperContent>(connection)
		.map_err(|err| println!("error when query paper content {}", err))
		.and_then(|result| {
			result
				.get(0)
				.ok_or(())
				.map(|row| markdown_parser::parse_markdown2html_json_struct(&row.content))
		})
}

// insert new paper, don't need to insert id , default use uuid_v1;
// don't need create time, default use now();
// don't need record time
// don't need is_draft default true,
// don't need is_del default false,
// post new_paper_api
pub fn post_new_paper(
	param_title: &str,
	param_author: &str,
	param_content: &str,
	param_tags: &[String],
) -> Result<(), ()> {
	let connection = &*get_connection().get().expect("can't set connection");
	use self::papers::dsl::*;
	diesel::insert_into(papers)
		.values((
			title.eq(param_title),
			content.eq(param_content),
			author.eq(param_author),
			tags.eq(param_tags),
		))
		.execute(connection)
		.unwrap(); // FIXME: rm this unwrap
	Ok(())
}
