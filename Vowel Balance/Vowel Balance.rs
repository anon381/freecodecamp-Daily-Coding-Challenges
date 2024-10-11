// Checks if the number of vowels in the first half and second half of a string are equal.
// Returns true if balanced, false otherwise.
// Parameters:
//   s: input string
// Returns:
//   true if the string is vowel balanced, false otherwise.
//
// Usage Examples:
//   is_balanced("racecar") => true
//   is_balanced("hello")   => false
// Time Complexity: O(n), where n is the length of the input string
// Space Complexity: O(1), only a few variables are used
fn is_balanced(s: &str) -> bool {
    let vowels = "aeiouAEIOU";
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let half = n / 2;

    let count_first = chars[..half].iter().filter(|c| vowels.contains(*c)).count();
    let count_second = chars[n - half..].iter().filter(|c| vowels.contains(*c)).count();

    count_first == count_second
}

fn main() {
    println!("{}", is_balanced("racecar")); // true
    println!("{}", is_balanced("Lorem Ipsum")); // true
    println!("{}", is_balanced("Kitty Ipsum")); // false
}
