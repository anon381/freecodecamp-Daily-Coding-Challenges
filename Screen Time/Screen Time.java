// Java implementation
public class ScreenTime {
    public static boolean tooMuchScreenTime(int[] hours) {
        // Rule 1: any single day >= 10
        for (int h : hours) if (h >= 10) return true;
        // Rule 2: average of any 3 consecutive days >= 8
        for (int i = 0; i < hours.length - 2; i++) {
            if ((hours[i] + hours[i+1] + hours[i+2]) / 3.0 >= 8) return true;
        }
        // Rule 3: weekly average >= 6
        int total = 0;
        for (int h : hours) total += h;
        if (total / 7.0 >= 6) return true;
        return false;
    }
    public static void main(String[] args) {
        System.out.println(tooMuchScreenTime(new int[]{7,8,9,5,6,7,8})); // true
        System.out.println(tooMuchScreenTime(new int[]{5,5,5,5,5,5,5})); // false
    }
}
