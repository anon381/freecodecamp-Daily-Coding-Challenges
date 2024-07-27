// Time Complexity: O(n), where n is the length of the input string
// Space Complexity: O(1), only a few variables are used
fn is_valid_number(s: &str, base: u32) -> bool {
    for ch in s.chars() {
        let val = match ch.to_ascii_uppercase() {
            '0'..='9' => ch as u32 - '0' as u32,
            'A'..='Z' => 10 + (ch as u32 - 'A' as u32),
            _ => return false,
        };
        if val >= base {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", is_valid_number("10101", 2));   // true
    println!("{}", is_valid_number("10201", 2));   // false
    println!("{}", is_valid_number("ABC", 16));    // true
    println!("{}", is_valid_number("5G3F8F", 16)); // false
    println!("{}", is_valid_number("5G3F8F", 17)); // true
}
