# This function checks if all characters in the input string are unique (case-sensitive).
# It returns True if no character repeats, and False otherwise. Uses a set to track seen characters.
# Time Complexity: O(n), where n is the length of the string
# Space Complexity: O(n), due to the set or hash structure
def all_unique(s: str) -> bool:
	"""
	Return True if all characters in s are unique (case-sensitive).
	Raises TypeError if input is not a string.
	"""
	if not isinstance(s, str):
		raise TypeError("Input must be a string.")
	return len(s) == len(set(s))

# Quick checks
if __name__ == "__main__":
	while True:
		s = input("Enter a string to check for unique characters (or 'exit' to quit): ")
		if s.lower() == 'exit':
			break
		try:
			result = all_unique(s)
			print(f"All characters unique: {result}")
		except Exception as e:
			print(f"Error: {e}")
	for inp, expected in tests:
		print(inp, all_unique(inp), "EXPECTED:", expected)
