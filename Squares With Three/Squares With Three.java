// Time Complexity: O(n)
// Space Complexity: O(1)

public class SquaresWithThree {
    public static int squaresWithDigit(int n, char digit) {
        if (n <= 0) {
            throw new IllegalArgumentException("Input must be a positive integer.");
        }
        if (!Character.isDigit(digit)) {
            throw new IllegalArgumentException("Digit must be a numeric character.");
        }
        int count = 0;
        for (int i = 1; i <= n; i++) {
            if (String.valueOf(i * i).indexOf(digit) != -1) {
                count++;
            }
        }
        return count;
    }

    public static void main(String[] args) {
        int n = 10;
        char digit = '3';
        System.out.println("Count: " + squaresWithDigit(n, digit));
    }
}
