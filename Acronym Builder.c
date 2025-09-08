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
	char temp[256];
	strcpy(temp, phrase);
	char* word = strtok(temp, " ");
	int i = 0, pos = 0;
	while (word) {
		if (i > 0 && is_ignore_word(word)) {
			word = strtok(NULL, " ");
			i++;
			continue;
		}
		acronym[pos++] = toupper(word[0]);
		word = strtok(NULL, " ");
		i++;
	}
	acronym[pos] = '\0';
}
