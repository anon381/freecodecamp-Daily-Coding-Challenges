// Time Complexity: O(n)
// Space Complexity: O(1)

fn squares_with_digit(n: u32, digit: char) -> u32 {
    if n == 0 {
        panic!("Input must be a positive integer.");
    }
    if !digit.is_ascii_digit() {
        panic!("Digit must be a numeric character.");
    }
    let mut count = 0;
    for i in 1..=n {
        if (i * i).to_string().contains(digit) {
            count += 1;
        }
    }
    count
}

fn main() {
    let n = 10;
    let digit = '3';
    println!("Count: {}", squares_with_digit(n, digit));
}
    // This function counts numbers from 1 to n whose squares contain the digit '3'. It demonstrates Rust's string conversion and pattern matching capabilities for digit analysis.
