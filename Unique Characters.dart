// Time Complexity: O(n), where n is the length of the string
bool allUnique(String s) {
	// Return true if all characters in s are unique (case-sensitive)
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
