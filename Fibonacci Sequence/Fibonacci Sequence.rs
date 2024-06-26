
// Time Complexity: O(n), where n is the length of the sequence to generate.
// Space Complexity: O(n), for storing the output sequence of length n.
fn fibonacci_sequence(start: [i64; 2], length: usize) -> Vec<i64> {
    if length == 0 {
        return vec![];
    } else if length == 1 {
        return vec![start[0]];
    } else if length == 2 {
        return vec![start[0], start[1]];
    }

    let mut seq = vec![start[0], start[1]];
    while seq.len() < length {
        let n = seq.len();
        let next = seq[n - 1] + seq[n - 2];
        seq.push(next);
    }
    seq
}

fn main() {
    println!("{:?}", fibonacci_sequence([0, 1], 10));
    println!("{:?}", fibonacci_sequence([123456789, 987654321], 5));
}
