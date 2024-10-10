void main() {
	String message = "Hello, World!";
	int shift = 3;
	String decoded = decode(message, shift);
	print('Original: $message');
	print('Decoded: $decoded');
}
// Time Complexity: O(n), where n is the length of the message
// Space Complexity: O(n), due to result string
// Decodes a message by shifting alphabetic characters backwards by 'shift'.
String decode(String message, int shift) {
	// Create a StringBuffer to store the decoded result
	StringBuffer result = StringBuffer();
	// Iterate over each character in the message
	for (int i = 0; i < message.length; i++) {
		var ch = message[i];
		// Check if the character is alphabetic
		if (RegExp(r'[a-zA-Z]').hasMatch(ch)) {
			// Determine base ASCII value ('A' for uppercase, 'a' for lowercase)
			int base = ch.toUpperCase() == ch ? 'A'.codeUnitAt(0) : 'a'.codeUnitAt(0);
			// Shift the character backwards by 'shift' and wrap around the alphabet
			int decodedChar = ((ch.codeUnitAt(0) - base - shift) % 26 + 26) % 26 + base;
			// Append the decoded character to the result
			result.write(String.fromCharCode(decodedChar));
		} else {
			// Non-alphabetic characters are appended unchanged
			result.write(ch);
		}
	}
	// Return the final decoded string
	return result.toString();
	}

	// This function decodes a message by shifting each alphabetic character backwards by the specified shift value.
	// Non-alphabetic characters remain unchanged. The result is a decoded string using a Caesar cipher in reverse.
}
