// This function checks if all characters in the input string are unique (case-sensitive).
// It returns true if no character repeats, and false otherwise. Uses a HashSet to track seen characters.
// Time Complexity: O(n), where n is the length of the string
// Space Complexity: O(n), due to the set or hash structure
use std::collections::HashSet;

fn all_unique(s: &str) -> bool {
	if s.is_empty() {
		eprintln!("Error: Input string is empty.");
		return false;
	}
	let mut seen = HashSet::new();
	for ch in s.chars() {
		if !seen.insert(ch) {
			return false;
		}
	}
	true
}

use std::io::{self, Write};

fn main() {
	loop {
		print!("Enter a string to check for unique characters (or 'exit' to quit): ");
		io::stdout().flush().unwrap();
		let mut input = String::new();
		if io::stdin().read_line(&mut input).is_err() { break; }
		let input = input.trim();
		if input.eq_ignore_ascii_case("exit") { break; }
		let result = all_unique(input);
		println!("All characters unique: {}", result);
	}
}
