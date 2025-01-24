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
		java.util.Scanner scanner = new java.util.Scanner(System.in);
		while (true) {
			System.out.print("Enter a string to check for unique characters (or 'exit' to quit): ");
			String input = scanner.nextLine();
			if (input.equalsIgnoreCase("exit")) break;
			try {
				boolean result = allUnique(input);
				System.out.println("All characters unique: " + result);
			} catch (Exception e) {
				System.out.println("Error: " + e.getMessage());
			}
		}
		scanner.close();
	}
}
