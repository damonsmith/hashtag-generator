use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

pub struct Dictionary {
    arena: Arena,
    start_node_id: usize,
    pub word_count: usize
}

impl Dictionary {

    pub fn new(dict_file: &str) -> Dictionary {
        
        let re = Regex::new(r"[\d\&]").unwrap();

        let mut arena = Arena::new();
        let start_node_id = arena.new_node(' ');
        let file = File::open(dict_file).unwrap();
        let reader = BufReader::new(file);
        let mut word_count = 0;
        for line in reader.lines() {
            let word = line.unwrap();
            // ignore words that contain numbers or ampersands
            // but allow apostrophes
            if re.is_match(word.as_str()) {
                continue;
            }

            if word.len() < 2 {
                continue;
            }

            let mut current_node_id = start_node_id;
            for c in word.chars() {
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

        Self { 
            arena: arena,
            start_node_id: start_node_id,
            word_count: word_count
        }
    }
    
    pub fn get_words_starting_with(&self, word: &str) {
        let mut current_node_id = self.start_node_id;
        for c in word.chars() {
            if c < 'A' || c > 'z' {
                continue;
            };
            current_node_id = match self.arena.get(current_node_id).children.get(&c) {
                Some(&id) => id,
                None => return,
            };
        }
    
        let word_without_last_char: String = word.chars().take(word.len() - 1).collect();
    
        self.get_all_word_endings(word_without_last_char.as_str(), current_node_id);
    }

    pub fn get_words_in_string(&self, sentence: &str, pos: usize, parsed_words: String) {
        let mut current_node_id = self.start_node_id;
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
            current_node_id = match self.arena.get(current_node_id).children.get(&c) {
                Some(n) => *n,
                None => break
            };
            let current_node = self.arena.get(current_node_id);
            
            // if it is not a word ending continue
            words.push(c);

            if current_node.word_ending {
                
                // if we are at a word ending but the next char is 
                // one of the child nodes then we need to create a new
                // call starting from the new word
                if current_node.children.contains_key(&next_c) {
                    let mut cloned_words = words.clone();
                    cloned_words.push(' ');
                    self.get_words_in_string(sentence, i+1, cloned_words);
                }
                // if there are no longer words then reset the current node
                // to the top of the tree again and add a space
                else {
                    words.push(' ');
                    current_node_id = self.start_node_id;
                }
            }
            
        }
        println!("{}", words);
    }
    
    pub fn get_all_word_endings(&self, word: &str, current_node_id: usize) {
        let current_node = self.arena.get(current_node_id);
        let word_next = format!("{}{}", word, current_node.data);
        if current_node.word_ending {
            println!("n: {}", word_next);
        }
        for node_id in current_node.children.values() {
            self.get_all_word_endings(word_next.as_str(), *node_id);
        }
    }
    
    pub fn get_sub_words(&self, word: &str) {
        let mut word_set: HashSet<&str> = HashSet::new();
        let mut current_node_id = self.start_node_id;
        word.chars().enumerate().for_each(|(pos, c)| {
            if c < 'A' || c > 'z' {
                return;
            };
            current_node_id = match self.arena.get(current_node_id).children.get(&c) {
                Some(&id) => id,
                None => return,
            };
            if self.arena.get(current_node_id).word_ending {
                word_set.insert(word.get(0..pos).unwrap());
                println!("{}", word.get(0..pos).unwrap());
            }
        });
    }
    
    pub fn contains(&self, word: &str) -> bool {
        let mut current_node_id = self.start_node_id;
        for c in word.chars() {
            if c < 'A' || c > 'z' {
                continue;
            };
            current_node_id = match self.arena.get(current_node_id).children.get(&c) {
                Some(&id) => id,
                None => return false,
            };
            println!("letter: {}", c);
        }
    
        true
    }
}

struct Arena {
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
