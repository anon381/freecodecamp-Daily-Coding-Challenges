// This function checks if all characters in the input string are unique (case-sensitive).
// It returns true if no character repeats, and false otherwise. Uses a set to track seen characters.
// Time Complexity: O(n), where n is the length of the string
// Space Complexity: O(n), due to the set or hash structure
bool allUnique(String s) {
	if (s is! String) {
		throw ArgumentError('Input must be a String.');
	}
	var seen = <String>{};
	for (var ch in s.split('')) {
		if (seen.contains(ch)) return false;
		seen.add(ch);
	}
	return true;
}

import 'dart:io';

void main() {
	while (true) {
		stdout.write("Enter a string to check for unique characters (or 'exit' to quit): ");
		String? input = stdin.readLineSync();
		if (input == null || input.toLowerCase() == 'exit') break;
		try {
			bool result = allUnique(input);
			print("All characters unique: $result");
		} catch (e) {
			print("Error: $e");
		}
	}
		print("${test[0]} ${allUnique(test[0])} EXPECTED: ${test[1]}");
	}
}
