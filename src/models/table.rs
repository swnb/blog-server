use super::schema::papers;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[table_name = "papers"]
pub struct Paper {
	id: i32,
	title: String,
	pub content: String,
	author: String,
	last_change_time: String,
	create_time: String,
	tags: String,
	hash: String,
}

#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[table_name = "papers"]
pub struct PaperInfo {
	title: String,
	author: String,
	last_change_time: String,
	create_time: String,
	tags: String,
	hash: String,
}

#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[table_name = "papers"]
pub struct PaperContent {
	title: String,
	pub content: String,
}
