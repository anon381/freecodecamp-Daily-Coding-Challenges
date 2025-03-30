// Time Complexity: O(n)
// Space Complexity: O(1)
int squaresWithDigit(int n, String digit) {
  if (n <= 0) {
    throw ArgumentError('Input must be a positive integer.');
  }
  if (digit.length != 1 || int.tryParse(digit) == null) {
    throw ArgumentError('Digit must be a single numeric character.');
  }
  int count = 0;
  for (int i = 1; i <= n; i++) {
    if ((i * i).toString().contains(digit)) {
      count++;
    }
  }
  return count;
}

void main() {
  int n = 10;
  String digit = '3';
  print('Count: [32m${squaresWithDigit(n, digit)}');
    // This function counts numbers from 1 to n whose squares contain the digit. It demonstrates Dart's string conversion and digit pattern analysis.
}
