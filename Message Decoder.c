int main() {
	const char* message = "Hello, World!";
	int shift = 3;
	char output[100];
	decode(message, shift, output);
	printf("Original: %s\n", message);
	printf("Decoded: %s\n", output);
	return 0;
}
/*
Time Complexity: O(n), where n is the length of the message
Space Complexity: O(n), due to output string
*/
#include <stdio.h>
#include <ctype.h>
#include <string.h>

void decode(const char* message, int shift, char* output) {
		// Get the length of the message
		int len = strlen(message);
		// Iterate over each character in the message
		for (int i = 0; i < len; i++) {
			char ch = message[i];
			// Check if the character is alphabetic
			if (isalpha(ch)) {
				// Determine base ASCII value ('A' for uppercase, 'a' for lowercase)
				char base = isupper(ch) ? 'A' : 'a';
				// Shift the character backwards by 'shift' and wrap around the alphabet
				char decoded = ((ch - base - shift + 26) % 26) + base;
				// Store the decoded character in the output
				output[i] = decoded;
			} else {
				// Non-alphabetic characters are stored unchanged
				output[i] = ch;
			}
		}
		// Null-terminate the output string
		output[len] = '\0';
	// This function decodes a message by shifting each alphabetic character backwards by the specified shift value.
	// Non-alphabetic characters remain unchanged. The result is a decoded string using a Caesar cipher in reverse.
}
