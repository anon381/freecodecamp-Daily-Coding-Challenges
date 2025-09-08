// Time Complexity: O(n)
String buildAcronym(String phrase) {
	final ignoreWords = {'a', 'an', 'and', 'for', 'by', 'of'};
	final words = phrase.split(' ');
	final acronym = <String>[];
	for (var i = 0; i < words.length; i++) {
		final lw = words[i].toLowerCase();
		if (i > 0 && ignoreWords.contains(lw)) {
			continue;
		}
		acronym.add(words[i][0].toUpperCase());
	}
	return acronym.join();
}
