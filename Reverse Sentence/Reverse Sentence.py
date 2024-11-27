# Time Complexity: O(n), where n is the number of characters in the input string.
# Python implementation

def reverse_sentence(s: str) -> str:
    # Split by whitespace, automatically ignoring multiple spaces
    words = s.split()
    # Reverse the list
    words.reverse()
    # Join back with a single space
    return " ".join(words)

# Example usage
if __name__ == "__main__":
    print(reverse_sentence("hello   world!  this is   python"))  # Output: 'python is this world! hello'
