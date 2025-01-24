import java.util.HashSet;

public class UniqueCharacters {
	public static boolean allUnique(String s) {
		// Return true if all characters in s are unique (case-sensitive)
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
