// Time Complexity: O(n), where n is the number of days (typically 7).
// Space Complexity: O(1), only a constant amount of extra space is used.
// Rust implementation
fn too_much_screen_time(hours: &[i32]) -> bool {
    // Rule 1: any single day >= 10
    if hours.iter().any(|&h| h >= 10) {
        return true;
    }
    // Rule 2: average of any 3 consecutive days >= 8
    for i in 0..(hours.len() - 2) {
        if (hours[i] + hours[i+1] + hours[i+2]) as f64 / 3.0 >= 8.0 {
            return true;
        }
    }
    // Rule 3: weekly average >= 6
    let total: i32 = hours.iter().sum();
    if total as f64 / 7.0 >= 6.0 {
        return true;
    }
    false
}

fn main() {
    println!("{}", too_much_screen_time(&[7, 8, 9, 5, 6, 7, 8])); // true
    println!("{}", too_much_screen_time(&[5, 5, 5, 5, 5, 5, 5])); // false
}
