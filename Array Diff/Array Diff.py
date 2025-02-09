

def array_diff(arr1, arr2):
    # Time Complexity: O(n + m), where n = len(arr1), m = len(arr2)
    # Space Complexity: O(n + m), for storing sets and result
    set1 = set(arr1)  # Convert arr1 to a set
    set2 = set(arr2)  # Convert arr2 to a set
    diff = set1.symmetric_difference(set2)  # Find symmetric difference
    return sorted(diff)  # Return sorted result

# Input validation example
if __name__ == "__main__":
    arr1 = [1, 2, 3]
    arr2 = [3, 4, 5]
    print(array_diff(arr1, arr2))
