
def squares_with_three(n: int) -> int:
    count = 0
    for i in range(1, n + 1):
        if '3' in str(i * i):
            count += 1
    return count

# Input validation example
if __name__ == "__main__":
    n = 10
    print(f"Count: {squares_with_three(n)}")
