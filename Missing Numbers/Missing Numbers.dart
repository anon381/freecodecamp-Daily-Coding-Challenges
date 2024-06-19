// Dart implementation
List<int> findMissingNumbers(List<int> arr) {
  if (arr.isEmpty) return [];
  int n = arr.reduce((a, b) => a > b ? a : b);
  var nums = Set<int>.from(arr);
  return [for (var i = 1; i <= n; i++) if (!nums.contains(i)) i];
}

void main() {
  print(findMissingNumbers([1, 2, 4, 6, 6, 3, 7, 8])); // [5]
  print(findMissingNumbers([])); // []
}
