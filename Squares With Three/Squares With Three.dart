// Time Complexity: O(n)
// Space Complexity: O(1)
int squaresWithThree(int n) {
  int count = 0;
  for (int i = 1; i <= n; i++) {
    if ((i * i).toString().contains('3')) {
      count++;
    }
  }
  return count;
}

void main() {
  int n = 10;
  print('Count: [32m${squaresWithThree(n)}');
    // This function counts numbers from 1 to n whose squares contain the digit '3'. It demonstrates Dart's string conversion and digit pattern analysis.
}
