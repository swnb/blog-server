use super::schema::papers;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[table_name = "papers"]
pub struct Papers {
	id: i32,
	title: String,
	pub content: String,
	author: String,
	last_change_time: String,
	create_time: String,
	tags: String,
	index_hash: String,
}
