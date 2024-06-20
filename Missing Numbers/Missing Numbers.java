// Java implementation
import java.util.*;

public class MissingNumbers {
    public static List<Integer> findMissingNumbers(int[] arr) {
        if (arr.length == 0) return new ArrayList<>();
        int n = Arrays.stream(arr).max().getAsInt();
        Set<Integer> nums = new HashSet<>();
        for (int x : arr) nums.add(x);
        List<Integer> missing = new ArrayList<>();
        for (int i = 1; i <= n; i++) if (!nums.contains(i)) missing.add(i);
        return missing;
    }
    public static void main(String[] args) {
        System.out.println(findMissingNumbers(new int[]{1,2,4,6,6,3,7,8})); // [5]
        System.out.println(findMissingNumbers(new int[]{})); // []
    }
}
