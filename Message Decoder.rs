fn main() {
	let message = "Hello, World!";
	let shift = 3;
	let decoded = decode(message, shift);
	println!("Original: {}", message);
	println!("Decoded: {}", decoded);
}
// Time Complexity: O(n), where n is the length of the message
// Space Complexity: O(n), due to result string
pub fn decode(message: &str, shift: i32) -> String {
	// Create a String to store the decoded result
	let mut result = String::new();
	// Iterate over each character in the message
	for ch in message.chars() {
		// Check if the character is alphabetic
		if ch.is_ascii_alphabetic() {
			// Determine base ASCII value ('A' for uppercase, 'a' for lowercase)
			let base = if ch.is_ascii_uppercase() { b'A' } else { b'a' };
			// Shift the character backwards by 'shift' and wrap around the alphabet
			let decoded = ((ch as u8 - base + 26 - (shift as u8 % 26)) % 26) + base;
			// Append the decoded character to the result
			result.push(decoded as char);
		} else {
			// Non-alphabetic characters are appended unchanged
			result.push(ch);
		}
	}
	// Return the final decoded string
	result
	}

	// This function decodes a message by shifting each alphabetic character backwards by the specified shift value.
	// Non-alphabetic characters remain unchanged. The result is a decoded string using a Caesar cipher in reverse.
}
