#include <stdbool.h>
#include <math.h>

// Checks if a number is an "unnatural prime" (prime for both positive and negative values)
#include <stdbool.h>
#include <math.h>

/*
Time Complexity: O(sqrt(n))
*/
bool is_unnatural_prime(int n) {
	// 0 and ±1 are not prime
	if (n == 0 || n == 1 || n == -1) {
		return false;
	}
	int num = n < 0 ? -n : n; // handle negative numbers
	if (num < 2) {
		return false;
	}
	// check divisibility up to sqrt(num)
	int sqrt_num = (int)sqrt((double)num);
	for (int i = 2; i <= sqrt_num; i++) {
		if (num % i == 0) {
			return false;
		}
	}
	return true;
}
