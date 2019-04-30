mod html_parser;

use html_parser::{dom::Dom, parser::Parser as h_Parser};
use pulldown_cmark::{html, Event, Parser};
use serde_json;

// &quot; -> """
// &amp; -> "&"
// &lt;	-> "<"
// &gt; -> ">"
// &nbsp; -> " "

use lazy_static::lazy_static;

lazy_static! {
	static ref REGEX: Regex = Regex::new("&(quot|amp|lt|gt|nbsp);").unwrap();
}

use regex::{Captures, Regex};
// replace Escape Sequence into text
pub fn replace_escape_sequence(text: &str) -> std::borrow::Cow<'_, str> {
	REGEX.replace_all(text, |cap: &Captures| match &cap[0] {
		"&quot;" => "\"",
		"&amp;" => "&",
		"&lt;" => "<",
		"&gt;" => ">",
		"&nbsp;" => " ",
		_ => panic!("error in regex when replace Escape Sequence"),
	})
}

pub fn parse_markdown2html_json_struct(text: &str) -> String {
	let parser = Parser::new(text).map(|event| match event {
		Event::Text(text) => Event::Text(text.replace("\t", "    ").into()), // replace \t into  4 space
		_ => event,
	});
	// parse makrdown into html
	let mut text = String::from("");
	html::push_html(&mut text, parser);
	// parse html string into dom struct
	let mut parser = h_Parser::new(&mut text);
	parser.on_dom_insert(|dom: &mut Dom| {
		if dom.query_tag_name() == "content" {
			let attrs = dom.query_mut_attrs();
			if let Some(text) = attrs.get("text") {
				// replace escape string
				let text = replace_escape_sequence(text);
				attrs.insert(String::from("text"), text.to_string());
			}
		}
	});
	// println error message when error happen
	parser.parse().unwrap_or_else(|error_message| {
		println!("{}", error_message);
	});
	let result = parser.result();
	serde_json::to_string(result).unwrap()
}
