// Time Complexity: O(n^2)
// Space Complexity: O(n)
// Decodes a string by reversing substrings inside parentheses. For each closing parenthesis,
// the substring inside the matching parentheses is reversed in place.
fn decode(s: &str) -> String {
    let mut stack: Vec<char> = Vec::new();

    // Iterate through each character in the input string
    for c in s.chars() {
        // If the character is a closing parenthesis
        if c == ')' {
            let mut temp = Vec::new();
            // Pop characters from the stack until an opening parenthesis is found
            while let Some(&top) = stack.last() {
                // If the top of the stack is an opening parenthesis, remove it and break
                if top == '(' {
                    stack.pop(); // remove '('
                    break;
                }
                // Otherwise, pop the character and add it to temp
                temp.push(stack.pop().unwrap());
            }
            // Push the reversed substring (inside parentheses) back onto the stack
            stack.extend(temp);
        } else {
            // If not a closing parenthesis, push the character onto the stack
            stack.push(c);
        }
    }

    // Collect the characters from the stack into a string and return
    stack.iter().collect()
}

