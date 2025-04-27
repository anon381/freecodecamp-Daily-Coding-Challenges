import java.util.*;

public class RomanParser {
    public static int parseRomanNumeral(String s) {
        Map<Character, Integer> values = new HashMap<>();
        values.put('I', 1);
        values.put('V', 5);
        values.put('X', 10);
        values.put('L', 50);
        values.put('C', 100);
        values.put('D', 500);
        values.put('M', 1000);

        int total = 0;
        int i = 0;

        while (i < s.length()) {
            // Check if this char is smaller than the next one → subtract
            if (i + 1 < s.length() && values.get(s.charAt(i)) < values.get(s.charAt(i + 1))) {
                total += values.get(s.charAt(i + 1)) - values.get(s.charAt(i));
                i += 2;
            } else {
                total += values.get(s.charAt(i));
                i++;
            }
        }

        return total;
    }

   
}
