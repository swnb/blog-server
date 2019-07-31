use log::{info, trace};
use std::{
	env, fs,
	io::{BufRead, BufReader},
};

enum Mode {
	Dev,
	Pro,
}

fn parse_dot_env_file() {
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
				trace!("set key {} value {}", key, value);
			}
		}
	})
}

fn get_mode() -> Mode {
	match env::var("MODE") {
		Ok(ref mode) if mode == "develop" => Mode::Dev,
		Ok(ref mode) if mode == "product" => Mode::Pro,
		_ => {
			println!("env MODE must set with develop or product");
			std::process::exit(100);
		}
	}
}

pub fn set_env() {
	parse_dot_env_file();
	let mode = get_mode();
	match mode {
		Mode::Dev => {
			super::logger::set_logger("blog=trace,actix_web=info");
			info!("develop mode");
		}
		Mode::Pro => {
			super::logger::set_logger("blog=info,actix_web=info");
			info!("product mode");
		}
	}
}
