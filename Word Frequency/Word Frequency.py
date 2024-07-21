import re
from collections import Counter

def get_words(paragraph):
	"""
	Time Complexity: O(n^2), where n is the number of words (due to unique_words.contains and sorting).
	Space Complexity: O(n), for storing the word counts and unique words list.
	"""
	# Lowercase and remove punctuation
	text = re.sub(r"[,.!]", "", paragraph.lower())
	words = text.split()

	# Count frequencies
	counts = Counter(words)

	# Preserve first occurrence order
	unique_words = []
	for w in words:
		if w not in unique_words:
			unique_words.append(w)

	# Sort: frequency desc, then by first occurrence in text
	sorted_words = sorted(unique_words, key=lambda w: (-counts[w], unique_words.index(w)))

	# Return top 3
	return sorted_words[:3]
