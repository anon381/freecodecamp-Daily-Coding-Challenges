/*
Time Complexity: O(n), where n is the length of the message
Space Complexity: O(n), due to output string
*/
#include <stdio.h>
#include <ctype.h>
#include <string.h>

// Decodes a message by shifting alphabetic characters backwards by 'shift'.
void decode(const char* message, int shift, char* output) {
	int len = strlen(message);
	for (int i = 0; i < len; i++) {
		char ch = message[i];
		if (isalpha(ch)) {
			char base = isupper(ch) ? 'A' : 'a';
			char decoded = ((ch - base - shift + 26) % 26) + base;
			output[i] = decoded;
		} else {
			output[i] = ch;
		}
	}
	output[len] = '\0';
	// This function decodes a message by shifting each alphabetic character backwards by the specified shift value.
	// Non-alphabetic characters remain unchanged. The result is a decoded string using a Caesar cipher in reverse.
}
