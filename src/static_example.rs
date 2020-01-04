use std::collections::HashMap;

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
			data: data,
		});

		// Return the node identifier
		next_index
	}

	pub fn get(&mut self, id: usize) -> &mut Node {
		self.nodes.get_mut(id).unwrap()
	}
}

pub struct Node {
	pub children: HashMap<char, usize>,
	pub data: char,
}

fn main() {
	let arena = &mut Arena::new();
	let top_node_id = arena.new_node(' ');

	// dummy dict
	let dict = vec!["able", "abbey", "aardvark"];
	// populate dict
	for word in dict.iter() {
		let mut current_node_id = top_node_id;
		for c in word.chars() {
			if c < 'A' || c > 'z' {
				continue;
			};
			if !arena.get(current_node_id).children.contains_key(&c) {
				let new_node_id = arena.new_node(c);
				arena.get(current_node_id).children.insert(c, new_node_id);
				current_node_id = new_node_id;
			} else {
				current_node_id = match arena.get(current_node_id).children.get(&c) {
					Some(&id) => id,
					None => panic!("can't get node"),
				}
			}
		}
	}

	println!(
		"Word exists: {}",
		node_contains_string("able", arena, top_node_id)
	)
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
