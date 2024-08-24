// Checks if a number is an "unnatural prime" (prime for both positive and negative values)
// Time Complexity: O(sqrt(n))
pub fn is_unnatural_prime(n: i32) -> bool {
	// 0 and ±1 are not prime
	if n == 0 || n == 1 || n == -1 {
		return false;
	}
	let num = n.abs(); // handle negative numbers
	if num < 2 {
		return false;
	}
	// check divisibility up to sqrt(num)
	let sqrt_num = (num as f64).sqrt() as i32;
	for i in 2..=sqrt_num {
		if num % i == 0 {
			return false;
		}
	}
	true
}
