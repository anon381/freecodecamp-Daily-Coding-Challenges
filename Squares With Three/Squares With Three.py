# Time Complexity: O(n)
# Space Complexity: O(1)

def squares_with_three(n: int, digit: str = '3') -> int:
    if n <= 0:
        raise ValueError("Input must be a positive integer.")
    if not (isinstance(digit, str) and len(digit) == 1 and digit.isdigit()):
        raise ValueError("Digit must be a single character string representing a digit.")
    count = 0
    for i in range(1, n + 1):
        if digit in str(i * i):
            count += 1
    return count

# Input validation example
if __name__ == "__main__":
    n = 10
    digit = '3'
    print(f"Count: {squares_with_three(n, digit)}")
# This function iterates through numbers from 1 to n, checks if the square of each number contains the digit '3', and counts how many such numbers exist. Useful for digit pattern analysis in squares.
