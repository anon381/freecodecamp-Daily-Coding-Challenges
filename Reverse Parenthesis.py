# Decodes a string by reversing substrings inside parentheses. For each closing parenthesis, the substring inside the matching parentheses is reversed in place.
# Space Complexity: O(n)
# Time Complexity: O(n^2)
def decode(s):
    stack = []

    for char in s:
        if char == ')':
            # pop until matching '('
            temp = []
            while stack and stack[-1] != '(':
                temp.append(stack.pop())
            stack.pop()  # remove '('
            # push reversed substring back to stack
            stack.extend(temp)
        else:
            stack.append(char)

    return ''.join(stack)

