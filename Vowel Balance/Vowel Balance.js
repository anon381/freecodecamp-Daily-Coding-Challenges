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
function isBalanced(s) {
  if (!s || s.length === 0) {
    return false;
  }
  const vowels = "aeiouAEIOU";
  const n = s.length;
  const half = Math.floor(n / 2);

  const first = s.slice(0, half);
  const second = s.slice(n - half);

  const count = str => [...str].filter(c => vowels.includes(c)).length;

  return count(first) === count(second);
}

console.log(isBalanced("racecar")); // true
console.log(isBalanced("Lorem Ipsum")); // true
console.log(isBalanced("Kitty Ipsum")); // false
