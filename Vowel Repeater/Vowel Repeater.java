public class RepeatVowels {

    public static String repeatVowels(String s) {
        String vowels = "aeiouAEIOU";
        StringBuilder result = new StringBuilder();
        int count = 0;

        for (char c : s.toCharArray()) {
            if (vowels.indexOf(c) != -1) {
                count++;
                for (int i = 0; i < count; i++) result.append(i == 0 ? c : Character.toLowerCase(c));
            } else result.append(c);
        }

        return result.toString();
    }

   
}
