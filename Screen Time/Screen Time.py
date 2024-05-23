# Time Complexity: O(n), where n is the number of days (typically 7).
# Space Complexity: O(1), only a constant amount of extra space is used.
def too_much_screen_time(hours):
    # Rule 1: any single day >= 10
    if any(h >= 10 for h in hours):
        return True

    # Rule 2: average of any 3 consecutive days >= 8
    for i in range(len(hours) - 2):
        if sum(hours[i:i+3]) / 3 >= 8:
            return True

    # Rule 3: weekly average >= 6
    if sum(hours) / 7 >= 6:
        return True

    return False

# Example usage
if __name__ == "__main__":
    print(too_much_screen_time([7, 8, 9, 5, 6, 7, 8]))  # Output: True
    print(too_much_screen_time([5, 5, 5, 5, 5, 5, 5]))  # Output: False
