// Decodes a message by shifting alphabetic characters backwards by 'shift'.
String decode(String message, int shift) {
	StringBuffer result = StringBuffer();
	for (int i = 0; i < message.length; i++) {
		var ch = message[i];
		if (RegExp(r'[a-zA-Z]').hasMatch(ch)) {
			int base = ch.toUpperCase() == ch ? 'A'.codeUnitAt(0) : 'a'.codeUnitAt(0);
			int decodedChar = ((ch.codeUnitAt(0) - base - shift) % 26 + 26) % 26 + base;
			result.write(String.fromCharCode(decodedChar));
		} else {
			result.write(ch);
		}
	}
	return result.toString();
}
