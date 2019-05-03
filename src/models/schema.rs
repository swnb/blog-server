table! {
	papers (id) {
		id -> Integer,
		title -> Varchar,
		content -> Text,
		author -> Varchar,
		last_change_time -> Varchar,
		create_time	 -> Varchar,
		tags -> Varchar,
		index_hash -> Varchar,
	}
}
