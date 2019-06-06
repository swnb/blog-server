// paper info
table! {
	papers (id) {
		id -> Integer,
		title -> Varchar,
		content -> Text,
		author -> Varchar,
		last_change_time -> Varchar,
		create_time	 -> Varchar,
		tags -> Varchar,
		hash -> Varchar,
	}
}

// tag with paper_id
table!{
	paper_tags (id){
		id -> Integer,
		tag -> Varchar,
	}
}
