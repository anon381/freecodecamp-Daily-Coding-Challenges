# Time Complexity: O(n^2), where n is the number of words (due to unique search and sorting).
# Space Complexity: O(n), for storing the word structs and temporary buffers.
#include <stdio.h>
#include <string.h>
#include <ctype.h>

#define MAX_WORDS 100
#define MAX_WORD_LEN 32

typedef struct {
	char word[MAX_WORD_LEN];
	int count;
	int first_index;
} WordFreq;

// Helper to lowercase and remove punctuation
void normalize(char *dst, const char *src) {
	int j = 0;
	for (int i = 0; src[i]; i++) {
		char c = tolower(src[i]);
		if (c != ',' && c != '.' && c != '!') {
			dst[j++] = c;
		}
	}
	dst[j] = '\0';
}

int find_word(WordFreq *arr, int n, const char *word) {
	for (int i = 0; i < n; i++) {
		if (strcmp(arr[i].word, word) == 0) return i;
	}
	return -1;
}

void get_words(const char *paragraph) {
	char text[1024], *token;
	normalize(text, paragraph);
	WordFreq words[MAX_WORDS];
	int n = 0, idx = 0;
	token = strtok(text, " \t\n");
	while (token && n < MAX_WORDS) {
		int pos = find_word(words, n, token);
		if (pos == -1) {
			strncpy(words[n].word, token, MAX_WORD_LEN);
			words[n].count = 1;
			words[n].first_index = idx;
			n++;
		} else {
			words[pos].count++;
		}
		idx++;
		token = strtok(NULL, " \t\n");
	}
	// Sort by count desc, then first_index asc
	for (int i = 0; i < n-1; i++) {
		for (int j = i+1; j < n; j++) {
			if (words[j].count > words[i].count ||
				(words[j].count == words[i].count && words[j].first_index < words[i].first_index)) {
				WordFreq tmp = words[i]; words[i] = words[j]; words[j] = tmp;
			}
		}
	}
	// Print top 3
	for (int i = 0; i < n && i < 3; i++) {
		printf("%s\n", words[i].word);
	}
}

int main() {
	get_words("Hello world! Hello, code. Hello code world.");
	return 0;
}
