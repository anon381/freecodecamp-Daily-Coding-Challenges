# Time Complexity: O(n)
def build_acronym(phrase: str) -> str:
	# Words to ignore unless they are the very first word
	ignore_words = {"a", "an", "and", "for", "by", "of"}
	words = phrase.split()
	acronym = []
	for i, word in enumerate(words):
		lw = word.lower()
		if i > 0 and lw in ignore_words:
			continue
		acronym.append(word[0].upper())
	return "".join(acronym)
