# Time Complexity: O(n)
# Space Complexity: O(1)

def squares_with_three(n: int, digit: str = '3') -> int:
    # Check if n is a positive integer
    if n <= 0:
        raise ValueError("Input must be a positive integer.")
    # Check if digit is a single numeric character
    if not (isinstance(digit, str) and len(digit) == 1 and digit.isdigit()):
        raise ValueError("Digit must be a single character string representing a digit.")
    count = 0  # Initialize the counter
    for i in range(1, n + 1):  # Loop from 1 to n
        if digit in str(i * i):  # Check if digit is in the square of i
            count += 1  # Increment the counter if condition is met
    return count  # Return the final count

# Input validation example
if __name__ == "__main__":
    n = 10  # Example input value
    digit = '3'  # Example digit to search for
    print(f"Count: {squares_with_three(n, digit)}")  # Print the result
# This function iterates through numbers from 1 to n, checks if the square of each number contains the digit '3', and counts how many such numbers exist. Useful for digit pattern analysis in squares.
