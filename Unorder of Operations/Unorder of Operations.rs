fn evaluate(numbers: &[i32], operators: &[char]) -> i32 {
    if numbers.is_empty() {
        return 0;
    }

    let mut result = numbers[0];

    for (i, &num) in numbers.iter().enumerate().skip(1) {
        let op = operators[(i - 1) % operators.len()];

        match op {
            '+' => result += num,
            '-' => result -= num,
            '*' => result *= num,
            '/' => result /= num, // integer division
            '%' => result %= num,
            _ => (), // ignore unknown operators
        }
    }

    result
}

fn main() {
    let numbers = vec![10, 5, 2];
    let operators = vec!['+', '*'];
    let result = evaluate(&numbers, &operators);
    println!("Result: {}", result); // Output: 30
}

