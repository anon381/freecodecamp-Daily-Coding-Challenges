// Space Complexity: O(n)
// Time Complexity: O(n)
String buildAcronym(String phrase) {
	// Set of words to ignore unless they are the first word
	final ignoreWords = {'a', 'an', 'and', 'for', 'by', 'of'};
	// Split the phrase into words
	final words = phrase.split(' ');
	// List to store acronym characters
	final acronym = <String>[];
	// Iterate through each word in the phrase
	for (var i = 0; i < words.length; i++) {
		// Convert word to lowercase for comparison
		final lw = words[i].toLowerCase();
		// Ignore the word if it's in the ignore list and not the first word
		if (i > 0 && ignoreWords.contains(lw)) {
			continue;
		}
		// Add the uppercase first character of the word to the acronym
		acronym.add(words[i][0].toUpperCase());
	}
	// Return the final acronym as a string
	return acronym.join();
}

// This function builds an acronym from a given phrase, ignoring certain common words unless they are the first word.
