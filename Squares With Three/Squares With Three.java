// Time Complexity: O(n)
// Space Complexity: O(1)

public class SquaresWithThree {
    // Method to count numbers whose squares contain the given digit
    public static int squaresWithDigit(int n, char digit) {
        // Validate that n is positive
        if (n <= 0) {
            throw new IllegalArgumentException("Input must be a positive integer.");
        }
        // Validate that digit is a numeric character
        if (!Character.isDigit(digit)) {
            throw new IllegalArgumentException("Digit must be a numeric character.");
        }
        int count = 0; // Initialize counter
        for (int i = 1; i <= n; i++) { // Loop from 1 to n
            // Check if digit is present in the square of i
            if (String.valueOf(i * i).indexOf(digit) != -1) {
                count++; // Increment counter if condition is met
            }
        }
        return count; // Return the final count
    }

    public static void main(String[] args) {
        int n = 10; // Example input value
        char digit = '3'; // Example digit to search for
        // Print the result
        System.out.println("Count: " + squaresWithDigit(n, digit));
    }
}
