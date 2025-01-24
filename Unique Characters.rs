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

fn main() {
	let tests = [
		("abc", true),
		("aA", true),
		("QwErTy123!@", true),
		("~!@#$%^&*()_+", true),
		("hello", false),
		("freeCodeCamp", false),
		("!@#*$%^&*()aA", false),
	];
	for (inp, expected) in tests.iter() {
		println!("{} {} EXPECTED: {}", inp, all_unique(inp), expected);
	}
}
