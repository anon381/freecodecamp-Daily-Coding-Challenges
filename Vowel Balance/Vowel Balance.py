# Time Complexity: O(n), where n is the length of the input string
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
