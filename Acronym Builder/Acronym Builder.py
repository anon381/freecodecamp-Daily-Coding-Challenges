# Space Complexity: O(n)
# Time Complexity: O(n)
def build_acronym(phrase: str) -> str:
	# Set of words to ignore unless they are the first word
	ignore_words = {"a", "an", "and", "for", "by", "of"}
	# Split the phrase into words
	words = phrase.split()
	# List to store acronym characters
	acronym = []
	# Iterate through each word in the phrase
	for i, word in enumerate(words):
		# Convert word to lowercase for comparison
		lw = word.lower()
		# Ignore the word if it's in the ignore list and not the first word
		if i > 0 and lw in ignore_words:
			continue
		# Add the uppercase first character of the word to the acronym
		acronym.append(word[0].upper())
	# Return the final acronym as a string
	return "".join(acronym)

# This function builds an acronym from a given phrase, ignoring certain common words unless they are the first word.
