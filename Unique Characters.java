// This function checks if all characters in the input string are unique (case-sensitive).
// It returns true if no character repeats, and false otherwise. Uses a HashSet to track seen characters.
// Time Complexity: O(n), where n is the length of the string
// Space Complexity: O(n), due to the set or hash structure
import java.util.HashSet;

public class UniqueCharacters {
	public static boolean allUnique(String s) {
		if (s == null) {
			throw new IllegalArgumentException("Input string cannot be null.");
		}
		HashSet<Character> seen = new HashSet<>();
		for (char ch : s.toCharArray()) {
			if (!seen.add(ch)) return false;
		}
		return true;
	}

	public static void main(String[] args) {
		String[][] tests = {
			{"abc", "true"},
			{"aA", "true"},
			{"QwErTy123!@", "true"},
			{"~!@#$%^&*()_+", "true"},
			{"hello", "false"},
			{"freeCodeCamp", "false"},
			{"!@#*$%^&*()aA", "false"},
		};
		for (String[] test : tests) {
			System.out.println(test[0] + " " + allUnique(test[0]) + " EXPECTED: " + test[1]);
		}
	}
}
