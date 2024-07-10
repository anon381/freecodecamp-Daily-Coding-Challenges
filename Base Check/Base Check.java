public class BaseCheck {
    public static boolean isValidNumber(String s, int base) {
        s = s.toUpperCase();
        for (char c : s.toCharArray()) {
            int val;
            if (Character.isDigit(c)) {
                val = c - '0';
            } else if (c >= 'A' && c <= 'Z') {
                val = 10 + (c - 'A');
            } else {
                return false; // invalid char
            }
            if (val >= base) return false;
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(isValidNumber("10101", 2));   // true
        System.out.println(isValidNumber("10201", 2));   // false
        System.out.println(isValidNumber("ABC", 16));    // true
        System.out.println(isValidNumber("5G3F8F", 16)); // false
        System.out.println(isValidNumber("5G3F8F", 17)); // true
    }
}
