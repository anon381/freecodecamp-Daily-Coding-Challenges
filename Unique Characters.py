def all_unique(s: str) -> bool:
	"""
	Return True if all characters in s are unique (case-sensitive).
	"""
	return len(s) == len(set(s))

# Quick checks
if __name__ == "__main__":
	tests = [
		("abc", True),
		("aA", True),
		("QwErTy123!@", True),
		("~!@#$%^&*()_+", True),
		("hello", False),
		("freeCodeCamp", False),
		("!@#*$%^&*()aA", False),
	]
	for inp, expected in tests:
		print(inp, all_unique(inp), "EXPECTED:", expected)
