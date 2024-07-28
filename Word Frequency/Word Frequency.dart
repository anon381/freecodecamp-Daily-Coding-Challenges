// Time Complexity: O(n^2), where n is the number of words (due to unique search and sorting).
// Space Complexity: O(n), for storing the word counts and unique words list.
import 'dart:collection';

List<String> getWords(String paragraph) {
	// Lowercase and remove punctuation
	var text = paragraph.toLowerCase().replaceAll(RegExp(r'[,.!]'), '');
	var words = text.split(RegExp(r'\s+'));

	// Count frequencies
	var counts = <String, int>{};
	for (var w in words) {
		counts[w] = (counts[w] ?? 0) + 1;
	}

	// Preserve first occurrence order
	var uniqueWords = <String>[];
	for (var w in words) {
		if (!uniqueWords.contains(w)) uniqueWords.add(w);
	}

	// Sort: frequency desc, then by first occurrence
	uniqueWords.sort((a, b) {
		var cmp = counts[b]!.compareTo(counts[a]!);
		if (cmp != 0) return cmp;
		return uniqueWords.indexOf(a).compareTo(uniqueWords.indexOf(b));
	});

	// Return top 3
	return uniqueWords.take(3).toList();
}

void main() {
	print(getWords('Hello world! Hello, code. Hello code world.'));
}
