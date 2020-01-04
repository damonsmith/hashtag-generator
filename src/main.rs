
extern crate regex;
mod dictionary;

fn main() {
	let dict = dictionary::Dictionary::new("/usr/share/dict/cracklib-small");
	//let dict = dictionary::Dictionary::new("dict.txt");

	println!(
		"Word exists: {}",
		dict.contains("pe")
	);

	dict.get_sub_words("abstentions");
	println!("word count: {}", dict.word_count);
	dict.get_words_starting_with("abs");
	dict.get_words_in_string("wereallyessay", 0, String::new());
}


