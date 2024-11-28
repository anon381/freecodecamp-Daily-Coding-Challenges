// Time Complexity: O(n), where n is the number of characters in the input string.
// Space Complexity: O(n), for storing the words and output.
// Rust implementation
fn reverse_sentence(s: &str) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    words.into_iter().rev().collect::<Vec<&str>>().join(" ")
}

fn main() {
    println!("{}", reverse_sentence("hello   world!  this is   rust")); // Output: 'rust is this world! hello'
}
