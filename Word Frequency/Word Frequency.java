import java.util.*;

public class WordFrequency {
	public static List<String> getWords(String paragraph) {
		// Lowercase and remove punctuation
		String text = paragraph.toLowerCase().replaceAll("[,.!]", "");
		String[] words = text.split("\\s+");

		// Count frequencies
		Map<String, Integer> counts = new HashMap<>();
		for (String w : words) counts.put(w, counts.getOrDefault(w, 0) + 1);

		// Preserve first occurrence order
		List<String> uniqueWords = new ArrayList<>();
		for (String w : words) if (!uniqueWords.contains(w)) uniqueWords.add(w);

		// Sort: frequency desc, then by first occurrence
		uniqueWords.sort((a, b) -> {
			int cmp = counts.get(b) - counts.get(a);
			if (cmp != 0) return cmp;
			return Integer.compare(uniqueWords.indexOf(a), uniqueWords.indexOf(b));
		});

		// Return top 3
		return uniqueWords.subList(0, Math.min(3, uniqueWords.size()));
	}

	public static void main(String[] args) {
		System.out.println(getWords("Hello world! Hello, code. Hello code world."));
	}
}
