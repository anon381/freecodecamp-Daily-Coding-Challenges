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
	char input[1024];
	while (1) {
		printf("Enter a string to check for unique characters (or 'exit' to quit): ");
		if (!fgets(input, sizeof(input), stdin)) break;
		// Remove newline
		size_t len = strlen(input);
		if (len > 0 && input[len-1] == '\n') input[len-1] = '\0';
		if (strcmp(input, "exit") == 0) break;
		bool result = all_unique(input);
		printf("All characters unique: %s\n", result ? "true" : "false");
	}
	return 0;
}
