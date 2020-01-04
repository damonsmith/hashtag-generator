use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct Arena {
	nodes: Vec<Node>,
}

impl Arena {
	pub fn new() -> Arena {
		Self { nodes: Vec::new() }
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

fn main() {
	let arena = &mut Arena::new();
	let top_node_id = arena.new_node(' ');

	let file = File::open("/usr/share/dict/cracklib-small").unwrap();
	let reader = BufReader::new(file);
	let mut word_count = 0;
	for word in reader.lines() {
		let mut current_node_id = top_node_id;
		for c in word.unwrap().chars() {
			if c < 'A' || c > 'z' {
				continue;
			};
			if !arena.get(current_node_id).children.contains_key(&c) {
				let new_node_id = arena.new_node(c);
				arena
					.get_mut(current_node_id)
					.children
					.insert(c, new_node_id);
				current_node_id = new_node_id;
			} else {
				current_node_id = match arena.get(current_node_id).children.get(&c) {
					Some(&id) => id,
					None => panic!("can't get node"),
				}
			}
		}
		arena.get_mut(current_node_id).word_ending = true;
		word_count += 1;
	}

	println!(
		"Word exists: {}",
		node_contains_string("abstentions", arena, top_node_id)
	);

	get_sub_words("abstentions", arena, top_node_id);
	println!("word count: {}", word_count);
	println!("node count: {}", arena.nodes.len());
	get_words_starting_with("abs", arena, top_node_id);
}

fn get_words_starting_with(word: &str, nodes: &Arena, top: usize) {
	let mut current_node_id = top;
	for c in word.chars() {
		if c < 'A' || c > 'z' {
			continue;
		};
		current_node_id = match nodes.get(current_node_id).children.get(&c) {
			Some(&id) => id,
			None => return,
		};
	}

	let word_without_last_char: String = word.chars().take(word.len() - 1).collect();

	get_all_word_endings(word_without_last_char.as_str(), nodes, current_node_id);
}

fn get_all_word_endings(word: &str, nodes: &Arena, top: usize) {
	let current_node = nodes.get(top);
	let word_next = format!("{}{}", word, current_node.data);
	if current_node.word_ending {
		println!("n: {}", word_next);
	}
	for node_id in current_node.children.values() {
		get_all_word_endings(word_next.as_str(), nodes, *node_id);
	}
}

// fn get_overlapping_words(word: &str, nodes: &mut Arena, top: usize) {
// 	for i in 1..4 {
// 		word.get(word.len() - i..word.len()).unwrap();
// 	}
// }

fn get_sub_words(word: &str, nodes: &mut Arena, top: usize) {
	let mut word_set: HashSet<&str> = HashSet::new();
	let mut current_node_id = top;
	word.chars().enumerate().for_each(|(pos, c)| {
		if c < 'A' || c > 'z' {
			return;
		};
		current_node_id = match nodes.get(current_node_id).children.get(&c) {
			Some(&id) => id,
			None => return,
		};
		if nodes.get(current_node_id).word_ending {
			word_set.insert(word.get(0..pos).unwrap());
			println!("{}", word.get(0..pos).unwrap());
		}
	});
}

fn node_contains_string(word: &str, nodes: &mut Arena, top: usize) -> bool {
	let mut current_node_id = top;
	for c in word.chars() {
		if c < 'A' || c > 'z' {
			continue;
		};
		current_node_id = match nodes.get(current_node_id).children.get(&c) {
			Some(&id) => id,
			None => return false,
		};
		println!("letter: {}", c);
	}

	true
}
