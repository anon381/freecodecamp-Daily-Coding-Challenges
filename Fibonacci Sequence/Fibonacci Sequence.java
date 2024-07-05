import java.util.*;

public class FibonacciSequence {
    public static List<Long> fibonacciSequence(long[] start, int length) {
        List<Long> seq = new ArrayList<>();
        if (length == 0) return seq;
        if (length == 1) {
            seq.add(start[0]);
            return seq;
        }
        if (length == 2) {
            seq.add(start[0]);
            seq.add(start[1]);
            return seq;
        }
        seq.add(start[0]);
        seq.add(start[1]);
        while (seq.size() < length) {
            int n = seq.size();
            seq.add(seq.get(n - 1) + seq.get(n - 2));
        }
        return seq;
    }

    public static void main(String[] args) {
        System.out.println(fibonacciSequence(new long[]{0, 1}, 10));
        System.out.println(fibonacciSequence(new long[]{123456789L, 987654321L}, 5));
    }
}
