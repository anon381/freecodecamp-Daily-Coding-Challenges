// Time Complexity: O(n)
import java.util.*;

public class AcronymBuilder {
	public static String buildAcronym(String phrase) {
		Set<String> ignoreWords = new HashSet<>(Arrays.asList("a", "an", "and", "for", "by", "of"));
		String[] words = phrase.split(" ");
		StringBuilder acronym = new StringBuilder();
		for (int i = 0; i < words.length; i++) {
			String lw = words[i].toLowerCase();
			if (i > 0 && ignoreWords.contains(lw)) {
				continue;
			}
			acronym.append(Character.toUpperCase(words[i].charAt(0)));
		}
		return acronym.toString();
	}
}
