// Time Complexity: O(n^2), where n is the number of words (due to unique search and sorting).
use std::collections::HashMap;

fn get_words(paragraph: &str) -> Vec<String> {
	// Lowercase and remove punctuation
	let text = paragraph.to_lowercase().replace([',', '.', '!'], "");
	let words: Vec<&str> = text.split_whitespace().collect();

	// Count frequencies
	let mut counts = HashMap::new();
	for w in &words {
		*counts.entry(*w).or_insert(0) += 1;
	}

	// Preserve first occurrence order
	let mut unique_words = Vec::new();
	for w in &words {
		if !unique_words.contains(w) {
			unique_words.push(*w);
		}
	}

	// Sort: frequency desc, then by first occurrence
	unique_words.sort_by(|a, b| {
		let cmp = counts[b].cmp(&counts[a]);
		if cmp != std::cmp::Ordering::Equal {
			cmp
		} else {
			let ia = words.iter().position(|&x| x == *a).unwrap();
			let ib = words.iter().position(|&x| x == *b).unwrap();
			ia.cmp(&ib)
		}
	});

	// Return top 3
	unique_words.into_iter().take(3).map(|s| s.to_string()).collect()
}

fn main() {
	println!("{:?}", get_words("Hello world! Hello, code. Hello code world."));
}
