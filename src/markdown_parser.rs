// use self::html_parser;
use pulldown_cmark::{html, Event, Parser};

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
		"&gt" => ">",
		"&nbsp;" => " ",
		_ => panic!("error in regex when replace Escape Sequence"),
	})
}

pub fn parse_markdown2html(text: &str) -> &str {
	let parser = Parser::new(text).map(|event| match event {
		Event::Text(text) => Event::Text(text.replace("\t", "    ").into()), // replace \t into space
		_ => event,
	});
	// html::push_html(&mut html_output, parser);
	""
}
