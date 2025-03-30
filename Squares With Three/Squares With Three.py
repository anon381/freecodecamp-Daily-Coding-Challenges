# Time Complexity: O(n)
# Space Complexity: O(1)

def squares_with_three(n: int) -> int:
    if n <= 0:
        raise ValueError("Input must be a positive integer.")
    count = 0
    for i in range(1, n + 1):
        if '3' in str(i * i):
            count += 1
    return count

# Input validation example
if __name__ == "__main__":
    n = 10
    print(f"Count: {squares_with_three(n)}")
# This function iterates through numbers from 1 to n, checks if the square of each number contains the digit '3', and counts how many such numbers exist. Useful for digit pattern analysis in squares.
