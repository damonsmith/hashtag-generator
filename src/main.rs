mod dictionary;
use std::env;
use substring::Substring;

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
		match req.url.query() {
			Some(query) => {
				let trimmed_query = match query.strip_prefix("q=") {
					Some(s) => s,
					None => "",
				};
				let mut sentences: Vec<String> = Vec::new();
				dict.get_words_in_string(trimmed_query, 0, String::new(), &mut sentences);
				let mut overlaps: Vec<String> = Vec::new();
				dict.get_overlapping_words(query, 6, &mut overlaps);
				let suggested: Vec<&String> = overlaps.iter().take(10).rev().collect();

				let resp = json!({
					"sentences": sentences,
					"suggested": suggested,
				});
				Ok(Response::with((status::Ok, resp.to_string())))	
			},
			None => Ok(Response::with(status::BadRequest)),
		}
	})
	.http("localhost:3000")
	.unwrap();
}
