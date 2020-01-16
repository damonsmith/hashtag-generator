mod dictionary;
use std::env;

extern crate serde_json;
use serde_json::json;

extern crate iron;
use iron::prelude::*;
use iron::status;

fn main() {
	let dict_file;

	let args: Vec<String> = env::args().collect();
	let switch_arg = match args.get(2) {
		Some(s) => s,
		None => "",
	};

	let dict_file_arg = match args.get(3) {
		Some(s) => s,
		None => "",
	};
	if switch_arg == "-f" && dict_file_arg.len() > 0 {
		dict_file = dict_file_arg;
	} else {
		dict_file = "engmix.txt";
	}

	let dict = dictionary::Dictionary::new(dict_file);

	Iron::new(move |req: &mut Request| {
		let mut sentences: Vec<String> = Vec::new();
		let suggested = match req.url.query() {
			Some(query) => {
				dict.get_words_in_string(query, 0, String::new(), &mut sentences);
				dict.get_overlapping_words(query, 6)
					.iter()
					.rev()
					.collect()
			},
			None => return Ok(Response::with(status::BadRequest)),
		};

		let resp = json!({
			"sentences": sentences,
			"suggested": suggested,
		});

		Ok(Response::with((status::Ok, resp.to_string())))
	})
	.http("localhost:3000")
	.unwrap();
}
