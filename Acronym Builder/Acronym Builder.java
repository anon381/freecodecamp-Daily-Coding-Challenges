// Space Complexity: O(n)
// Time Complexity: O(n)
import java.util.*;

public class AcronymBuilder {
	public static String buildAcronym(String phrase) {
		// Set of words to ignore unless they are the first word
		Set<String> ignoreWords = new HashSet<>(Arrays.asList("a", "an", "and", "for", "by", "of"));
		// Split the phrase into words
		String[] words = phrase.split(" ");
		// StringBuilder to construct the acronym
		StringBuilder acronym = new StringBuilder();
		// Iterate through each word in the phrase
		for (int i = 0; i < words.length; i++) {
			// Convert word to lowercase for comparison
			String lw = words[i].toLowerCase();
			// Ignore the word if it's in the ignore list and not the first word
			if (i > 0 && ignoreWords.contains(lw)) {
				continue;
			}
			// Add the uppercase first character of the word to the acronym
			acronym.append(Character.toUpperCase(words[i].charAt(0)));
		}
		// Return the final acronym as a string
		return acronym.toString();
	}

	// This function builds an acronym from a given phrase, ignoring certain common words unless they are the first word.
}
