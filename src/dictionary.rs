use regex::Regex;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct Dictionary {
	nodes: Vec<Node>,
	pub word_count: usize,
}

impl Dictionary {
	pub fn new(dict_file: &str) -> Dictionary {
		let re = Regex::new(r"[\d\&]").unwrap();

		
		let mut instance = Self {
			nodes: Vec::new(),
			word_count: 0,
		};

		instance.new_node(' ');

		let file = match File::open(dict_file) {
			Ok(f) => f,
			Err(e) => panic!(
				"ERROR-CANT-OPEN-DICT-FILE\nUnfortunately I've encountered an error while starting up, and that is I can't find the dictionary file at {}. The underlying error I received from the system is: {}",
				dict_file, e
			),
		};
		let reader = BufReader::new(file);
		for line in reader.lines() {
			let word = line.unwrap();
			// ignore words that contain numbers or ampersands
			// but allow apostrophes
			if re.is_match(word.as_str()) {
				continue;
			}

			if word.len() < 3 {
				continue;
			}
			instance.add_word_to_dict(&word);
			instance.word_count += 1;
		}

		instance
	}

	pub fn add_word_to_dict(&mut self, word: &String) {
		let mut current_node_id = 0;
		for c in word.chars() {
			if c < 'A' || c > 'z' {
				continue;
			};
			if !self.get(current_node_id).children.contains_key(&c) {
				let new_node_id = self.new_node(c);
				self
					.get_mut(current_node_id)
					.children
					.insert(c, new_node_id);
				current_node_id = new_node_id;
			} else {
				current_node_id = match self.get(current_node_id).children.get(&c) {
					Some(&id) => id,
					None => panic!("can't get node"),
				}
			}
		}
		self.get_mut(current_node_id).word_ending = true;

	}

	pub fn get_words_starting_with(&self, word: &str) -> Vec<String> {
		let mut current_node_id = 0;
		for c in word.chars() {
			if c < 'A' || c > 'z' {
				continue;
			};
			current_node_id = match self.get(current_node_id).children.get(&c) {
				Some(&id) => id,
				None => return Vec::new(),
			};
		}
		let word_without_last_char: String = word.chars().take(word.len() - 1).collect();

		let mut words: Vec<String> = Vec::new();
		self.get_all_word_endings(word_without_last_char.as_str(), current_node_id, &mut words);
		words
	}

	pub fn get_words_in_string(
		&self,
		sentence: &str,
		pos: usize,
		parsed_words: String,
		sentences: &mut Vec<String>,
	) {
		let mut current_node_id = 0;
		let mut words = parsed_words.clone();
		//let prefix: String = sentence.chars().take(pos).collect();
		//words.push_str(prefix.as_str());

		for i in pos..sentence.len() {
			let c = sentence.chars().nth(i).unwrap();
			let next_c = match sentence.chars().nth(i + 1) {
				Some(c) => c,
				None => ' ',
			};
			if c < 'A' || c > 'z' {
				continue;
			};
			// move along to the node for the current char
			current_node_id = match self.get(current_node_id).children.get(&c) {
				Some(n) => *n,
				None => break,
			};
			let current_node = self.get(current_node_id);

			// if it is not a word ending continue
			words.push(c);

			if current_node.word_ending {
				// if we are at a word ending but the next char is
				// one of the child nodes then we need to create a new
				// call starting from the new word
				if current_node.children.contains_key(&next_c) {
					let mut cloned_words = words.clone();
					cloned_words.push(' ');
					self.get_words_in_string(sentence, i + 1, cloned_words, sentences);
				}
				// if there are no longer words then reset the current node
				// to the top of the tree again and add a space
				else {
					words.push(' ');
					current_node_id = 0;
				}
			}
		}
		sentences.push(words);
	}

	pub fn get_all_word_endings(
		&self,
		word: &str,
		current_node_id: usize,
		words: &mut Vec<String>,
	) {
		let current_node = self.get(current_node_id);
		let word_next = format!("{}{}", word, current_node.data);
		if current_node.word_ending {
			words.push(word_next.clone());
		}
		for node_id in current_node.children.values() {
			self.get_all_word_endings(word_next.as_str(), *node_id, words);
		}
	}

	pub fn get_sub_words(&self, word: &str) {
		let mut word_set: HashSet<&str> = HashSet::new();
		let mut current_node_id = 0;
		word.chars().enumerate().for_each(|(pos, c)| {
			if c < 'A' || c > 'z' {
				return;
			};
			current_node_id = match self.get(current_node_id).children.get(&c) {
				Some(&id) => id,
				None => return,
			};
			if self.get(current_node_id).word_ending {
				word_set.insert(word.get(0..pos).unwrap());
			}
		});
	}

	pub fn contains(&self, word: &str) -> bool {
		let mut current_node_id = 0;
		for c in word.chars() {
			if c < 'A' || c > 'z' {
				continue;
			};
			current_node_id = match self.get(current_node_id).children.get(&c) {
				Some(&id) => id,
				None => return false,
			};
		}

		true
	}

	pub fn get_overlapping_words(&self, sentence: &str, max_overlap: usize) -> Vec<String> {
		let mut list: Vec<String> = Vec::new();

		for l in 1..cmp::min(max_overlap, sentence.len()) {
			let prefix: String = sentence.chars().skip(sentence.len() - l).take(l).collect();

			self.get_words_starting_with(prefix.as_str())
				.iter()
				.for_each(|word| {
					let suffix_of_word: String = word.chars().skip(prefix.len()).collect();
					if self.contains(suffix_of_word.as_str()) {
						list.push(word.clone());
					}
				})
		}
		list
	}

	pub fn new_node(&mut self, data: char) -> usize {
		// Get the next free index
		let next_index = self.nodes.len();

		// Push the node into the arena
		self.nodes.push(Node {
			children: HashMap::new(),
			word_ending: false,
			data: data,
		});

		// Return the node identifier
		next_index
	}

	pub fn get_mut(&mut self, id: usize) -> &mut Node {
		self.nodes.get_mut(id).unwrap()
	}

	pub fn get(&self, id: usize) -> &Node {
		self.nodes.get(id).unwrap()
	}

}

pub struct Node {
	pub children: HashMap<char, usize>,
	pub word_ending: bool,
	pub data: char,
}
