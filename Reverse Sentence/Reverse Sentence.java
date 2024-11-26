// Time Complexity: O(n), where n is the number of characters in the input string.
// Space Complexity: O(n), for storing the words and output.
// Java implementation
import java.util.*;

public class ReverseSentence {
    public static String reverseSentence(String s) {
        String[] words = s.trim().split("\\s+");
        List<String> wordList = Arrays.asList(words);
        Collections.reverse(wordList);
        return String.join(" ", wordList);
    }
    public static void main(String[] args) {
        System.out.println(reverseSentence("hello   world!  this is   java")); // Output: 'java is this world! hello'
    }
}
