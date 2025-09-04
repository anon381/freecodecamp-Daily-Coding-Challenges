def find_duplicates(arr):
    counts = {}
    for num in arr:
        counts[num] = counts.get(num, 0) + 1
        duplicates = [num for num, count in counts.items() if count > 1]
    return sorted(duplicates)

