// Time Complexity: O(n), where n is the length of the message
// Space Complexity: O(n), due to result string
// Decodes a message by shifting alphabetic characters backwards by 'shift'.
public class MessageDecoder {
	public static String decode(String message, int shift) {
		// Create a StringBuilder to store the decoded result
		StringBuilder result = new StringBuilder();
		// Iterate over each character in the message
		for (char ch : message.toCharArray()) {
			// Check if the character is a letter
			if (Character.isLetter(ch)) {
				// Determine the base ASCII value ('A' for uppercase, 'a' for lowercase)
				char base = Character.isUpperCase(ch) ? 'A' : 'a';
				// Shift the character backwards by 'shift' and wrap around the alphabet
				char decodedChar = (char)(((ch - base - shift + 26) % 26) + base);
				// Append the decoded character to the result
				result.append(decodedChar);
			} else {
				// Non-alphabetic characters are appended unchanged
				result.append(ch);
			}
		}
		// Return the final decoded string
		return result.toString();
	}
	// This function decodes a message by shifting each alphabetic character backwards by the specified shift value.
	// Non-alphabetic characters remain unchanged. The result is a decoded string using a Caesar cipher in reverse.
}
