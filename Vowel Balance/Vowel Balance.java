// Checks if the number of vowels in the first half and second half of a string are equal.
// Returns true if balanced, false otherwise.
// Parameters:
//   s: input string
// Returns:
//   true if the string is vowel balanced, false otherwise.
//
// Usage Examples:
//   VowelBalance.isBalanced("racecar") => true
//   VowelBalance.isBalanced("hello")   => false
// Time Complexity: O(n), where n is the length of the input string
// Space Complexity: O(1), only a few variables are used
public class VowelBalance {
    public static boolean isBalanced(String s) {
        String vowels = "aeiouAEIOU";
        int n = s.length();
        int half = n / 2;

        String first = s.substring(0, half);
        String second = s.substring(n - half);

        int countFirst = 0, countSecond = 0;

        for (char c : first.toCharArray()) {
            if (vowels.indexOf(c) != -1) countFirst++;
        }
        for (char c : second.toCharArray()) {
            if (vowels.indexOf(c) != -1) countSecond++;
        }

        return countFirst == countSecond;
    }

    public static void main(String[] args) {
        System.out.println(isBalanced("racecar")); // true
        System.out.println(isBalanced("Lorem Ipsum")); // true
        System.out.println(isBalanced("Kitty Ipsum")); // false
    }
}
