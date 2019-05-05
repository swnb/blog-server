use std::{
	env, fs,
	io::{BufRead, BufReader},
};

pub fn set_env() {
	let current_dir = env::current_dir().expect("can't get current dir");

	let env_file_path = current_dir.join(".env");

	let file = match fs::File::open(env_file_path) {
		Ok(file) => file,
		Err(_) => return,
	};

	let file_reader = BufReader::new(file);

	let (lines, _): (Vec<_>, _) = file_reader.lines().partition(Result::is_ok);
	let lines: Vec<String> = lines.into_iter().map(Result::unwrap).collect();
	lines.into_iter().for_each(|line: String| {
		let line_char = line.chars();
		for (index, c) in line_char.enumerate() {
			if c == '=' {
				let key = &line[..index].trim();
				let value = &line[(index + 1)..].trim();
				env::set_var(key, value);
				println!("set key {} value {}", key, value);
			}
		}
	})
}
