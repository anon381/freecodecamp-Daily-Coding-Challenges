

public class ArrayDiff {
    // Time Complexity: O(n + m), where n = arr1.length, m = arr2.length
    public static int[] arrayDiff(int[] arr1, int[] arr2) {
        java.util.Set<Integer> set1 = new java.util.HashSet<>();
        java.util.Set<Integer> set2 = new java.util.HashSet<>();
        for (int i : arr1) set1.add(i);
        for (int i : arr2) set2.add(i);
        java.util.Set<Integer> diff = new java.util.HashSet<>(set1);
        diff.removeAll(set2);
        for (int i : set2) if (!set1.contains(i)) diff.add(i);
        return diff.stream().sorted().mapToInt(Integer::intValue).toArray();
    }
    public static void main(String[] args) {
        int[] arr1 = {1, 2, 3};
        int[] arr2 = {3, 4, 5};
        int[] result = arrayDiff(arr1, arr2);
        System.out.print("[");
        for (int i = 0; i < result.length; i++) {
            System.out.print(result[i] + (i < result.length - 1 ? ", " : ""));
        }
        System.out.println("]");
    }
}
