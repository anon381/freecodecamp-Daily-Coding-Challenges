// Checks if the number of vowels in the first half and second half of a string are equal.
// Returns true if balanced, false otherwise.
// Parameters:
//   s: input string
// Returns:
//   true if the string is vowel balanced, false otherwise.
//
// Usage Examples:
//   isBalanced("racecar") => true
//   isBalanced("hello")   => false
// Time Complexity: O(n), where n is the length of the input string
// Space Complexity: O(1), only a few variables are used
bool isBalanced(String s) {
  if (s == null || s.isEmpty) {
    return false;
  }
  const vowels = "aeiouAEIOU";
  int n = s.length;
  int half = n ~/ 2;

  String first = s.substring(0, half);
  String second = s.substring(n - half);

  int countFirst = first.split('').where((c) => vowels.contains(c)).length;
  int countSecond = second.split('').where((c) => vowels.contains(c)).length;

  return countFirst == countSecond;
}

void main() {
  print(isBalanced("racecar")); // true
  print(isBalanced("Lorem Ipsum")); // true
  print(isBalanced("Kitty Ipsum")); // false
}
