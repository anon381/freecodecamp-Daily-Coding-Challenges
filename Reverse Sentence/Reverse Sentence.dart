// Dart implementation
String reverseSentence(String s) {
  var words = s.split(RegExp(r'\s+')).where((w) => w.isNotEmpty).toList();
  return words.reversed.join(' ');
}

void main() {
  print(reverseSentence('hello   world!  this is   dart')); // Output: 'dart is this world! hello'
}
