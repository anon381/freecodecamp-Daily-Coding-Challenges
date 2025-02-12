// Checks if a number is an "unnatural prime" (prime for both positive and negative values)
// Time Complexity: O(sqrt(n))
// Space Complexity: O(1)
bool isUnnaturalPrime(int n) {
	// 0 and ±1 are not prime
	if (n == 0 || n == 1 || n == -1) {
		return false;
	}
	int num = n.abs(); // handle negative numbers
	if (num < 2) {
		return false;
	}
	// check divisibility up to sqrt(num)
	for (int i = 2; i <= num.sqrt(); i++) {
		if (num % i == 0) {
			return false;
		}
	}
	return true;
}
