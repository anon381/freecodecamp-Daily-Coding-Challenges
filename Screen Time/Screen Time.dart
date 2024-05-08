// Time Complexity: O(n), where n is the number of days (typically 7).
// Space Complexity: O(1), only a constant amount of extra space is used.
// Dart implementation
bool tooMuchScreenTime(List<int> hours) {
  // Rule 1: any single day >= 10
  if (hours.any((h) => h >= 10)) return true;
  // Rule 2: average of any 3 consecutive days >= 8
  for (var i = 0; i < hours.length - 2; i++) {
    if ((hours[i] + hours[i + 1] + hours[i + 2]) / 3 >= 8) return true;
  }
  // Rule 3: weekly average >= 6
  if (hours.reduce((a, b) => a + b) / 7 >= 6) return true;
  return false;
}

void main() {
  print(tooMuchScreenTime([7, 8, 9, 5, 6, 7, 8])); // Output: true
  print(tooMuchScreenTime([5, 5, 5, 5, 5, 5, 5])); // Output: false
}
