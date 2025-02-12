# Decodes a string by reversing substrings inside parentheses. For each closing parenthesis, the substring inside the matching parentheses is reversed in place.
# Space Complexity: O(n)
# Time Complexity: O(n^2)
def decode(s):
    stack = []

    # Iterate through each character in the input string
    for char in s:
        # If the character is a closing parenthesis
        if char == ')':
            # Prepare to collect characters to be reversed
            temp = []
            # Pop characters from the stack until an opening parenthesis is found
            while stack and stack[-1] != '(': 
                temp.append(stack.pop())  # Add popped character to temp
            stack.pop()  # Remove the opening parenthesis '(' from the stack
            # Push the reversed substring (inside parentheses) back onto the stack
            stack.extend(temp)
        else:
            # If not a closing parenthesis, push the character onto the stack
            stack.append(char)

    return ''.join(stack)

