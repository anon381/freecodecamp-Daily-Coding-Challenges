// Space Complexity: O(n)
// Time Complexity: O(n)
fn build_acronym(phrase: &str) -> String {
	// Array of words to ignore unless they are the first word
	let ignore_words = ["a", "an", "and", "for", "by", "of"];
	// Split the phrase into words
	let words: Vec<&str> = phrase.split_whitespace().collect();
	// String to store acronym characters
	let mut acronym = String::new();
	// Iterate through each word in the phrase
	for (i, word) in words.iter().enumerate() {
		// Convert word to lowercase for comparison
		let lw = word.to_lowercase();
		// Ignore the word if it's in the ignore list and not the first word
		if i > 0 && ignore_words.contains(&lw.as_str()) {
			continue;
		}
		// Add the uppercase first character of the word to the acronym
		acronym.push(word.chars().next().unwrap().to_ascii_uppercase());
	}
	// Return the final acronym as a string
	acronym
}

// This function builds an acronym from a given phrase, ignoring certain common words unless they are the first word.
