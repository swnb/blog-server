// paper info
table! {
	papers (id) {
		id -> Integer,
		title -> Varchar,
		content -> Text,
		author -> Varchar,
		last_change_time -> Varchar,
		create_time	 -> Varchar,
		hash -> Varchar,
	}
}

// tag with paper_id
table! {
	paper_tags (id){
		id -> Varchar,
		tag -> Varchar,
	}
}
