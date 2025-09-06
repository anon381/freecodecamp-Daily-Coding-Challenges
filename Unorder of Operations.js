function evaluate(numbers, operators) {
    if (numbers.length === 0) return 0;

    let result = numbers[0];

    for (let i = 1; i < numbers.length; i++) {
        const op = operators[(i - 1) % operators.length];
        const num = numbers[i];

        switch (op) {
            case '+':
                result += num;
                break;
            case '-':
                result -= num;
                break;
            case '*':
                result *= num;
                break;
            case '/':
                result = Math.floor(result / num); // integer division
                break;
            case '%':
                result %= num;
                break;
        }
    }

    return result;
}

