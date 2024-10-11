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
