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

void main() {
	var tests = [
		["abc", true],
		["aA", true],
		["QwErTy123!@", true],
		["~!@#$%^&*()_+", true],
		["hello", false],
		["freeCodeCamp", false],
		["!@#*$%^&*()aA", false],
	];
	for (var test in tests) {
		print("${test[0]} ${allUnique(test[0])} EXPECTED: ${test[1]}");
	}
}
