// Time Complexity: O(n^2), where n is the number of words (due to unique search and sorting).
// Space Complexity: O(n), for storing the word counts and unique words array.
function getWords(paragraph) {
	// Lowercase and remove punctuation
	let text = paragraph.toLowerCase().replace(/[,.!]/g, '');
	let words = text.split(/\s+/);

	// Count frequencies
	let counts = {};
	for (let w of words) counts[w] = (counts[w] || 0) + 1;

	// Preserve first occurrence order
	let uniqueWords = [];
	for (let w of words) if (!uniqueWords.includes(w)) uniqueWords.push(w);

	// Sort: frequency desc, then by first occurrence
	uniqueWords.sort((a, b) => {
		let cmp = counts[b] - counts[a];
		if (cmp !== 0) return cmp;
		return uniqueWords.indexOf(a) - uniqueWords.indexOf(b);
	});

	// Return top 3
	return uniqueWords.slice(0, 3);
}

console.log(getWords('Hello world! Hello, code. Hello code world.'));
