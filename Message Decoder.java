// Time Complexity: O(n), where n is the length of the message
// Space Complexity: O(n), due to result string
// Decodes a message by shifting alphabetic characters backwards by 'shift'.
public class MessageDecoder {
	public static String decode(String message, int shift) {
		StringBuilder result = new StringBuilder();
		for (char ch : message.toCharArray()) {
			if (Character.isLetter(ch)) {
				char base = Character.isUpperCase(ch) ? 'A' : 'a';
				char decodedChar = (char)(((ch - base - shift + 26) % 26) + base);
				result.append(decodedChar);
			} else {
				result.append(ch);
			}
		}
		return result.toString();
	}
	// This function decodes a message by shifting each alphabetic character backwards by the specified shift value.
	// Non-alphabetic characters remain unchanged. The result is a decoded string using a Caesar cipher in reverse.
}
