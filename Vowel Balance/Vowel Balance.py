# Checks if the number of vowels in the first half and second half of a string are equal.
# Returns True if balanced, False otherwise.
# Parameters:
#   s: input string
# Returns:
#   True if the string is vowel balanced, False otherwise.
#
# Usage Examples:
#   is_balanced("racecar") => True
#   is_balanced("hello")   => False
# Time Complexity: O(n), where n is the length of the input string
# Space Complexity: O(1), only a few variables are used
def is_balanced(s: str) -> bool:
    vowels = set("aeiouAEIOU")
    n = len(s)
    
    # If odd, ignore the middle char
    half = n // 2
    first_half = s[:half]
    second_half = s[-half:] if half > 0 else ""
    
    count_first = sum(1 for ch in first_half if ch in vowels)
    count_second = sum(1 for ch in second_half if ch in vowels)
    
    return count_first == count_second
