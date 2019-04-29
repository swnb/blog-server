mod html_parser;

use html_parser::{dom::Dom, parser::Parser as h_Parser};
use pulldown_cmark::{html, Event, Parser};
use serde::{Deserialize, Serialize};
use serde_json;

// &quot; -> """
// &amp; -> "&"
// &lt;	-> "<"
// &gt; -> ">"
// &nbsp; -> " "

use lazy_static::lazy_static;

use regex::{Captures, Regex};

lazy_static! {
	static ref REGEX: Regex = Regex::new("&(quot|amp|lt|gt|nbsp);").unwrap();
}

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
		Event::Text(text) => Event::Text(text.replace("\t", "    ").into()), // replace \t into space
		_ => event,
	});
	let mut text = String::from("");
	html::push_html(&mut text, parser);
	let mut p = h_Parser::new(&mut text);
	p.on_dom_insert(|dom: &mut Dom| {
		if dom.query_tag_name() == "content" {
			let attrs = dom.query_mut_attrs();
			if let Some(text) = attrs.get("text") {
				let text = replace_escape_sequence(text);
				attrs.insert(String::from("text"), text.to_string());
			}
		}
	});
	p.parse();
	let result = p.result();
	serde_json::to_string(result).unwrap()
}
