// Time Complexity: O(n), where n is the length of the message
// Decodes a message by shifting alphabetic characters backwards by 'shift'.
pub fn decode(message: &str, shift: i32) -> String {
	let mut result = String::new();
	for ch in message.chars() {
		if ch.is_ascii_alphabetic() {
			let base = if ch.is_ascii_uppercase() { b'A' } else { b'a' };
			let decoded = ((ch as u8 - base + 26 - (shift as u8 % 26)) % 26) + base;
			result.push(decoded as char);
		} else {
			result.push(ch);
		}
	}
	result
}
