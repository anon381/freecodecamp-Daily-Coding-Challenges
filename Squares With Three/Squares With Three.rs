// Time Complexity: O(n)
// Space Complexity: O(1)

// Function to count numbers whose squares contain the given digit
fn squares_with_digit(n: u32, digit: char) -> u32 {
    // Validate that n is positive
    if n == 0 {
        panic!("Input must be a positive integer.");
    }
    // Validate that digit is a numeric character
    if !digit.is_ascii_digit() {
        panic!("Digit must be a numeric character.");
    }
    let mut count = 0; // Initialize counter
    for i in 1..=n { // Loop from 1 to n
        // Check if digit is present in the square of i
        if (i * i).to_string().contains(digit) {
            count += 1; // Increment counter if condition is met
        }
    }
    count // Return the final count
}

fn main() {
    let n = 10; // Example input value
    let digit = '3'; // Example digit to search for
    // Print the result
    println!("Count: {}", squares_with_digit(n, digit));
}
// This function counts numbers from 1 to n whose squares contain the digit. It demonstrates Rust's string conversion and pattern matching capabilities for digit analysis.
