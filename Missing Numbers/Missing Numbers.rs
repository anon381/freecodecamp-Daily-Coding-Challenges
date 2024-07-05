// Rust implementation
// Time Complexity: O(n), where n is the length of the input array (for set creation and max search)
// Space Complexity: O(n), for storing the HashSet and the output vector.
use std::collections::HashSet;

fn find_missing_numbers(arr: &[i32]) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }
    let n = *arr.iter().max().unwrap();
    let nums: HashSet<_> = arr.iter().cloned().collect();
    (1..=n).filter(|i| !nums.contains(i)).collect()
}

fn main() {
    println!("{:?}", find_missing_numbers(&[1, 2, 4, 6, 6, 3, 7, 8])); // [5]
    println!("{:?}", find_missing_numbers(&[])); // []
}
