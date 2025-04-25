// Time Complexity: O(n)
// Space Complexity: O(1)

fn squares_with_three(n: u32) -> u32 {
    let mut count = 0;
    for i in 1..=n {
        if (i * i).to_string().contains('3') {
            count += 1;
        }
    }
    count
}

fn main() {
    let n = 10;
    println!("Count: {}", squares_with_three(n));
}
