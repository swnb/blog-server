use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dom {
	tag: String,
	attrs: HashMap<String, String>,
	children: Vec<Dom>,
}

impl Dom {
	// init dom struct
	pub fn new(tag: &str, attrs: HashMap<String, String>, children: Vec<Dom>) -> Dom {
		let tag = tag.to_owned();
		Dom {
			tag,
			attrs,
			children,
		}
	}

	pub fn new_text(text: &str) -> Dom {
		let mut attrs = HashMap::new();
		attrs.insert(String::from("text"), text.to_owned());
		Dom {
			tag: String::from("content"),
			attrs,
			children: vec![],
		}
	}

	// query tag name
	pub fn query_tag_name(&self) -> &str {
		&self.tag
	}

	// query all attrs
	pub fn query_attrs(&self) -> &HashMap<String, String> {
		&self.attrs
	}

	pub fn query_children(&self) -> &Vec<Dom> {
		&self.children
	}

	// append child dom node
	pub fn append_child(&mut self, dom: Dom) {
		self.children.push(dom);
	}
}
