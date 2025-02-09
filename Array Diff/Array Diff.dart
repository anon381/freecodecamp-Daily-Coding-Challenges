

int arrayDiff(List<int> arr1, List<int> arr2) {
  final set1 = Set<int>.from(arr1);
  final set2 = Set<int>.from(arr2);
  final diff = set1.difference(set2).union(set2.difference(set1));
  final result = diff.toList()..sort();
  return result;
}

void main() {
  final arr1 = [1, 2, 3];
  final arr2 = [3, 4, 5];
  print(arrayDiff(arr1, arr2));
}
