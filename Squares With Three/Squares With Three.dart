// Time Complexity: O(n)
// Space Complexity: O(1)
// Function to count numbers whose squares contain the given digit
int squaresWithDigit(int n, String digit) {
  // Validate that n is positive
  if (n <= 0) {
    throw ArgumentError('Input must be a positive integer.');
  }
  // Validate that digit is a single numeric character
  if (digit.length != 1 || int.tryParse(digit) == null) {
    throw ArgumentError('Digit must be a single numeric character.');
  }
  int count = 0; // Initialize counter
  for (int i = 1; i <= n; i++) { // Loop from 1 to n
    // Check if digit is present in the square of i
    if ((i * i).toString().contains(digit)) {
      count++; // Increment counter if condition is met
    }
  }
  return count; // Return the final count
}

void main() {
  int n = 10; // Example input value
  String digit = '3'; // Example digit to search for
  // Print the result
  print('Count: \u001b[32m${squaresWithDigit(n, digit)}');
}
// This function counts numbers from 1 to n whose squares contain the digit. It demonstrates Dart's string conversion and digit pattern analysis.
