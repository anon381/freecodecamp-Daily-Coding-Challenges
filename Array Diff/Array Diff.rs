

// Time Complexity: O(n + m), where n = arr1.len(), m = arr2.len()
fn array_diff(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    use std::collections::HashSet;
    let set1: HashSet<_> = arr1.iter().cloned().collect();
    let set2: HashSet<_> = arr2.iter().cloned().collect();
    let diff: HashSet<_> = set1.symmetric_difference(&set2).cloned().collect();
    let mut result: Vec<i32> = diff.into_iter().collect();
    result.sort();
    result
}

fn main() {
    let arr1 = [1, 2, 3];
    let arr2 = [3, 4, 5];
    let result = array_diff(&arr1, &arr2);
    println!("{:?}", result);
}
