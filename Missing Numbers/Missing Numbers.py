def find_missing_numbers(arr):
    if not arr:
        return []

    n = max(arr)
    nums = set(arr)  # remove duplicates
    missing = [i for i in range(1, n + 1) if i not in nums]
    return missing

# Example usage
if __name__ == "__main__":
    print(find_missing_numbers([1, 2, 4, 6, 6, 3, 7, 8]))  # Output: [5]
    print(find_missing_numbers([]))  # Output: []
