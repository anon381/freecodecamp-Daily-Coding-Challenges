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
