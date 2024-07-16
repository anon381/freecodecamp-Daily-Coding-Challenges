// Time Complexity: O(n), where n is the length of the input string
// Space Complexity: O(1), only a few variables are used
bool isValidNumber(String s, int base) {
  s = s.toUpperCase();
  for (int i = 0; i < s.length; i++) {
    String ch = s[i];
    int val;
    if (RegExp(r'[0-9]').hasMatch(ch)) {
      val = int.parse(ch);
    } else if (RegExp(r'[A-Z]').hasMatch(ch)) {
      val = 10 + (ch.codeUnitAt(0) - 'A'.codeUnitAt(0));
    } else {
      return false;
    }
    if (val >= base) return false;
  }
  return true;
}

void main() {
  print(isValidNumber("10101", 2));   // true
  print(isValidNumber("10201", 2));   // false
  print(isValidNumber("ABC", 16));    // true
  print(isValidNumber("5G3F8F", 16)); // false
  print(isValidNumber("5G3F8F", 17)); // true
}
