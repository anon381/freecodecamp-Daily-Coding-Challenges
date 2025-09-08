// Time Complexity: O(n)
fn build_acronym(phrase: &str) -> String {
	let ignore_words = ["a", "an", "and", "for", "by", "of"];
	let words: Vec<&str> = phrase.split_whitespace().collect();
	let mut acronym = String::new();
	for (i, word) in words.iter().enumerate() {
		let lw = word.to_lowercase();
		if i > 0 && ignore_words.contains(&lw.as_str()) {
			continue;
		}
		acronym.push(word.chars().next().unwrap().to_ascii_uppercase());
	}
	acronym
}
