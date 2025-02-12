# Space Complexity: O(n)
# Time Complexity: O(n)
#include <stdio.h>
#include <string.h>
#include <ctype.h>

int is_ignore_word(const char* word) {
	const char* ignore[] = {"a", "an", "and", "for", "by", "of"};
	for (int i = 0; i < 6; i++) {
		if (strcasecmp(word, ignore[i]) == 0) return 1;
	}
	return 0;
}

void build_acronym(const char* phrase, char* acronym) {
	// Copy the input phrase to a temporary buffer
	char temp[256];
	strcpy(temp, phrase);
	// Split the phrase into words
	char* word = strtok(temp, " ");
	int i = 0, pos = 0;
	// Iterate through each word in the phrase
	while (word) {
		// Ignore the word if it's in the ignore list and not the first word
		if (i > 0 && is_ignore_word(word)) {
			word = strtok(NULL, " ");
			i++;
			continue;
		}
		// Add the uppercase first character of the word to the acronym
		acronym[pos++] = toupper(word[0]);
		word = strtok(NULL, " ");
		i++;
	}
	// Null-terminate the acronym string
	acronym[pos] = '\0';
}

// This function builds an acronym from a given phrase, ignoring certain common words unless they are the first word.
