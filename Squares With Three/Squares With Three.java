// Time Complexity: O(n)
// Space Complexity: O(1)

public class SquaresWithThree {
    public static int squaresWithThree(int n) {
        int count = 0;
        for (int i = 1; i <= n; i++) {
            if (String.valueOf(i * i).contains("3")) {
                count++;
            }
        }
            // This class provides a method to count numbers from 1 to n whose squares contain the digit '3'. It demonstrates string manipulation and digit pattern analysis in Java.
        return count;
    }

    public static void main(String[] args) {
        int n = 10;
        System.out.println("Count: " + squaresWithThree(n));
    }
}
