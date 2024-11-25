// Time Complexity: O(n), where n is the number of characters in the input string.
// Space Complexity: O(n), for storing the words and output.
// Dart implementation
String reverseSentence(String s) {
  var words = s.split(RegExp(r'\s+')).where((w) => w.isNotEmpty).toList();
  return words.reversed.join(' ');
}

void main() {
  print(reverseSentence('hello   world!  this is   dart')); // Output: 'dart is this world! hello'
}
