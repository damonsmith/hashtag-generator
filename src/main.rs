extern crate regex;
mod dictionary;
use std::env;
use std::io;
use std::io::prelude::*;

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
		dict_file = "/usr/share/dict/cracklib-small";
	}

	let dict = dictionary::Dictionary::new(dict_file);
	println!("ready, word count: {}, type exit to exit", dict.word_count);
	println!("contains ar: {}", dict.contains("ar"));
	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		let hash_line = line.unwrap();
		if hash_line == "exit" {
			break;
		}
		println!("*Permutations:");
		let mut sentences: Vec<String> = Vec::new();
		dict.get_words_in_string(hash_line.as_str(), 0, String::new(), &mut sentences);
		sentences.iter().for_each(|sentence| {
			println!("{}", sentence);
		});
		println!("*Suggested next words:");
		dict.get_overlapping_words(hash_line.as_str(), 6)
			.iter()
			.rev()
			.enumerate()
			.for_each(|(i, word)| {
				if i < 10 {
					println!("{}", word);
				}
			});
	}
}
