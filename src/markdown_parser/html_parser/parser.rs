use super::dom::Dom;

use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

type Callback = Box<Fn(&mut Dom)>;

pub struct Parser<'a> {
	root: Vec<Dom>,
	offset: usize,
	stack: Vec<Dom>,
	chars: Peekable<Chars<'a>>,
	cb_list: Vec<Callback>,
}

impl<'a> Parser<'a> {
	// get current offset
	fn get_offset(&self) -> usize {
		self.offset
	}

	// next char
	fn next(&mut self) -> char {
		self.offset += 1;
		self.chars.next().unwrap()
	}

	// 下一个是否满足这个条件
	fn is_next(&mut self, cond: impl Copy + Fn(&char) -> bool) -> bool {
		self.chars.peek().map_or(false, cond)
	}

	// 下一个满足条件就跳过
	fn is_next_then_jump(&mut self, cond: impl Copy + Fn(&char) -> bool) -> bool {
		if self.is_next(cond) {
			self.next();
			true
		} else {
			false
		}
	}
}

impl<'a> Parser<'a> {
	pub fn new(text: &str) -> Parser {
		let chars = text.chars().peekable();
		Parser {
			root: vec![],
			stack: vec![],
			offset: 0,
			chars,
			cb_list: vec![],
		}
	}

	pub fn result(&self) -> &Vec<Dom> {
		&self.root
	}

	pub fn parse(&mut self) -> Result<(), String> {
		self.stack = vec![];
		while self.chars.peek().is_some() {
			// 循环的查找
			self.clear_whitespace();
			if self.is_next_then_jump(|&c| c == '<') {
				self.clear_whitespace();
				if self.is_next_then_jump(|&c| c == '/') {
					// </
					self.clear_whitespace();
					let tag_name = self.read_tag_name();
					self.clear_whitespace();
					if !self.is_next_then_jump(|&c| c == '>') {
						let error_message =
							format!("{}:{}", "err in parse offset", self.get_offset());
						return Err(error_message);
					}

					// 跳出最近的节点
					let dom = self.stack.pop().unwrap(); // 这个必须是有的
					if dom.query_tag_name() != tag_name {
						let error_message =
							format!("{}:{}", "err in parse offset", self.get_offset());
						return Err(error_message);
					} else {
						self.insert_node(dom);
					}
				} else {
					// <
					self.clear_whitespace();
					let tag_name = self.read_tag_name();
					self.clear_whitespace();
					if self.is_next_then_jump(|&c| c == '/') {
						// <tag_name>
						self.is_next_then_jump(|&c| c == '>');
						// close tag
						let dom = Dom::new(&tag_name, HashMap::new(), vec![]);
						self.insert_node(dom);
					} else {
						// parse attr
						let attr = self.read_attrs();
						// >
						self.clear_whitespace();
						if !self.is_next_then_jump(|&c| c == '>') {
							let error_message =
								format!("{}:{}", "err in parse offset", self.get_offset());
							return Err(error_message);
						}

						let dom = Dom::new(&tag_name, attr, vec![]);
						self.insert_new_node(dom);
					}
				}
			} else {
				// parse pure text
				if let Some(text) = self.read_pure_text() {
					let dom = Dom::new_text(&text);
					self.insert_node(dom);
				} else {
					return Ok(());
				}
			}
		}
		Ok(())
	}

	// 清空空白的地方
	fn clear_whitespace(&mut self) {
		while self.is_next(|&c| c.is_whitespace() || c == '\n') {
			self.next();
		}
	}

	// 读取满足条件的字符串
	fn read_chars(&mut self, cond: impl Copy + Fn(&char) -> bool) -> String {
		let mut char_list = String::from("");
		while self.chars.peek().map_or(false, cond) {
			char_list.push(self.next());
		}
		char_list
	}

	fn read_tag_name(&mut self) -> String {
		let mut tag_name = String::from("");
		if self.is_next(|&c| c < 'A' || c > 'z') {
			return tag_name;
		}
		while self.is_next(|&c| (c >= 'A' && c <= 'z') || (c <= '9' && c >= '0')) {
			tag_name.push(self.next());
		}
		tag_name
	}

	fn read_attrs(&mut self) -> HashMap<String, String> {
		let mut attrs = HashMap::new();
		loop {
			self.clear_whitespace();
			if self.is_next(|&c| c == '>') {
				break;
			} else {
				let key = self.read_tag_name();
				self.clear_whitespace();
				if self.is_next_then_jump(|&c| c == '=') {
					self.clear_whitespace();
					if self.is_next_then_jump(|&c| c == '\'') {
						// ='
						let value = self.read_chars(|&c| c != '\'');
						self.next(); // jump '
						attrs.insert(key, value);
					} else if self.is_next_then_jump(|&c| c == '"') {
						// ="
						let value = self.read_chars(|&c| c != '"');
						self.next(); //jump '"'
						attrs.insert(key, value);
					} else {
						println!("{}:{}", "err in parse offset", self.get_offset());
					}
				} else {
					// 单独的属性
					attrs.insert(key, String::from(""));
				}
			}
		}
		attrs
	}

	fn read_pure_text(&mut self) -> Option<String> {
		let mut text = String::from("");
		loop {
			match self.chars.peek() {
				Some(&c) if c != '<' => {
					text.push(c);
					self.next();
				}
				Some(_) => return Some(text),
				None => return None,
			}
		}
	}

	fn insert_new_node(&mut self, node: Dom) {
		self.stack.push(node);
	}

	fn insert_node(&mut self, mut node: Dom) {
		for cb in &self.cb_list {
			cb(&mut node)
		}
		match self.stack.last_mut() {
			Some(parent) => parent.append_child(node),
			None => self.root.push(node),
		}
	}
}

// visitor
impl<'a> Parser<'a> {
	pub fn on_dom_insert<F>(&mut self, callback: F)
	where
		F: 'static + Send + Fn(&mut Dom),
	{
		self.cb_list.push(Box::new(callback));
	}
}

#[test]
fn test_add() {}
