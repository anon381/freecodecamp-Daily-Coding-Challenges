// Rust implementation
fn reverse_sentence(s: &str) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    words.into_iter().rev().collect::<Vec<&str>>().join(" ")
}

fn main() {
    println!("{}", reverse_sentence("hello   world!  this is   rust")); // Output: 'rust is this world! hello'
}
