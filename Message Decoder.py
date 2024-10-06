def decode(message: str, shift: int) -> str:
	result = []
	for ch in message:
		if ch.isalpha():
			base = ord('A') if ch.isupper() else ord('a')
			decoded_char = chr((ord(ch) - base - shift) % 26 + base)
			result.append(decoded_char)
		else:
			result.append(ch)
	return "".join(result)
