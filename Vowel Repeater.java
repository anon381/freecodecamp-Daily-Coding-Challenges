public class RepeatVowels {

    public static String repeatVowels(String s) {
        String vowels = "aeiouAEIOU";
        StringBuilder result = new StringBuilder();
        int count = 0;

        for (char c : s.toCharArray()) {
            if (vowels.indexOf(c) != -1) { // check if c is a vowel
                count++;
                result.append(c);
                for (int i = 1; i < count; i++) {
                    result.append(Character.toLowerCase(c));
                }
            } else {
                result.append(c);
            }
        }

        return result.toString();
    }

   
}
