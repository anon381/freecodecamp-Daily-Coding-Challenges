# Time Complexity: O(n), where n is the length of the input string
def is_valid_number(s: str, base: int) -> bool:
    s = s.upper()
    digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    
    valid_digits = digits[:base]
    
    return all(ch in valid_digits for ch in s)
