use super::schema::{paper_tags, papers};
use serde::{Deserialize, Serialize};

// tables papers
#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[table_name = "papers"]
pub struct Paper {
	id: i32,
	title: String,
	pub content: String,
	author: String,
	last_change_time: String,
	create_time: String,
	hash: String,
}

// select paper base info without content from table
#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[table_name = "papers"]
pub struct PaperInfo {
	title: String,
	author: String,
	last_change_time: String,
	create_time: String,
	hash: String,
}

// select paper content from papers
#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[table_name = "papers"]
pub struct PaperContent {
	title: String,
	pub content: String,
}

// table tag with paper id
#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[table_name = "paper_tags"]
pub struct PaperTags {
	id: String,
	tag: String,
}
