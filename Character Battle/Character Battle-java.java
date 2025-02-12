import java.util.*;

public class CharacterBattle {

    // Helper: get strength of a character
        // Returns the strength value of a character (a-z, A-Z, 0-9)
    public static int charStrength(char c) {
        if (c >= 'a' && c <= 'z') return c - 'a' + 1;
        if (c >= 'A' && c <= 'Z') return c - 'A' + 27;
        if (c >= '0' && c <= '9') return c - '0';
        return 0;
    }

        // Simulates a battle between two armies and returns the result
    public static String battle(String yourArmy, String oppArmy) {
        // Retreat conditions
        if (yourArmy.length() > oppArmy.length()) return "Opponent retreated";
        if (yourArmy.length() < oppArmy.length()) return "We retreated";

        int ourWins = 0, theirWins = 0;

        // Compare character by character
        for (int i = 0; i < yourArmy.length(); i++) {
            int y = charStrength(yourArmy.charAt(i));
            int o = charStrength(oppArmy.charAt(i));

            if (y > o) ourWins++;
            else if (o > y) theirWins++;
        }

        if (ourWins > theirWins) return "We won";
        if (ourWins < theirWins) return "We lost";
        return "It was a tie";
    }

   
}
