// Time Complexity: O(n^2)
// Space Complexity: O(n)
fn decode(s: &str) -> String {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        if c == ')' {
            let mut temp = Vec::new();
            while let Some(&top) = stack.last() {
                if top == '(' {
                    stack.pop(); // remove '('
                    break;
                }
                temp.push(stack.pop().unwrap());
            }
            // push reversed substring back
            stack.extend(temp);
        } else {
            stack.push(c);
        }
    }

    stack.iter().collect()
}

