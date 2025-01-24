// This function checks if all characters in the input string are unique (case-sensitive).
// It returns true if no character repeats, and false otherwise. Uses an array to track seen characters.
// Time Complexity: O(n), where n is the length of the string
// Space Complexity: O(n), due to the set or hash structure
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

bool all_unique(const char* s) {
	if (s == NULL) {
		fprintf(stderr, "Error: Input string is NULL.\n");
		return false;
	}
	int seen[256] = {0};
	for (int i = 0; s[i]; i++) {
		unsigned char ch = s[i];
		if (seen[ch]) return false;
		seen[ch] = 1;
	}
	return true;
}

int main() {
	const char* tests[][2] = {
		{"abc", "1"},
		{"aA", "1"},
		{"QwErTy123!@", "1"},
		{"~!@#$%^&*()_+", "1"},
		{"hello", "0"},
		{"freeCodeCamp", "0"},
		{"!@#*$%^&*()aA", "0"},
	};
	for (int i = 0; i < 7; i++) {
		printf("%s %d EXPECTED: %s\n", tests[i][0], all_unique(tests[i][0]), tests[i][1]);
	}
	return 0;
}
