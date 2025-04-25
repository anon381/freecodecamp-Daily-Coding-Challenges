
public class SquaresWithThree {
    public static int squaresWithThree(int n) {
        int count = 0;
        for (int i = 1; i <= n; i++) {
            if (String.valueOf(i * i).contains("3")) {
                count++;
            }
        }
        return count;
    }

    public static void main(String[] args) {
        int n = 10;
        System.out.println("Count: " + squaresWithThree(n));
    }
}
