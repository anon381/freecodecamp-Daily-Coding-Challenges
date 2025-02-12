"""
Time Complexity: O(sqrt(n))
Space Complexity: O(1)
"""
def is_unnatural_prime(n: int) -> bool:
	# 0 and ±1 are not prime
	if n in (0, 1, -1):
		return False
	num = abs(n)  # handle negative numbers
	if num < 2:
		return False
	# check divisibility up to sqrt(num)
	for i in range(2, int(num**0.5) + 1):
		if num % i == 0:
			return False
	return True
