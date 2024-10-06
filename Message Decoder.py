if __name__ == "__main__":
	message = "Hello, World!"
	shift = 3
	decoded = decode(message, shift)
	print(f"Original: {message}")
	print(f"Decoded: {decoded}")
"""
Time Complexity: O(n), where n is the length of the message
Space Complexity: O(n), due to result string
"""
def decode(message: str, shift: int) -> str:
	# List to store decoded characters
	result = []
	# Iterate over each character in the message
	for ch in message:
		# Check if the character is alphabetic
		if ch.isalpha():
			# Determine base ASCII value ('A' for uppercase, 'a' for lowercase)
			base = ord('A') if ch.isupper() else ord('a')
			# Shift the character backwards by 'shift' and wrap around the alphabet
			decoded_char = chr((ord(ch) - base - shift) % 26 + base)
			# Append the decoded character to the result
			result.append(decoded_char)
		else:
			# Non-alphabetic characters are appended unchanged
			result.append(ch)
	# Join the list into a final decoded string
	return "".join(result)

# This function decodes a message by shifting each alphabetic character backwards by the specified shift value.
# Non-alphabetic characters remain unchanged. The result is a decoded string using a Caesar cipher in reverse.
