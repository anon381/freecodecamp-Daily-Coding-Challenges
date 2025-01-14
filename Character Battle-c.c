# Space Complexity: O(1)
# Time Complexity: O(n)
#include <stdio.h>
#include <string.h>
#include <ctype.h>

# Returns the strength value of a character (a-z, A-Z, 0-9)
int char_strength(char c) {
    if (c >= 'a' && c <= 'z') return c - 'a' + 1;
    if (c >= 'A' && c <= 'Z') return c - 'A' + 27;
    if (c >= '0' && c <= '9') return c - '0';
    return 0;
}

# Simulates a battle between two armies and returns the result
const char* battle(const char* your, const char* opp) {
    int len1 = strlen(your);
    int len2 = strlen(opp);

    if (len1 > len2) return "Opponent retreated";
    if (len1 < len2) return "We retreated";

    int our_wins = 0, their_wins = 0;
    for (int i = 0; i < len1; i++) {
        int y = char_strength(your[i]);
        int o = char_strength(opp[i]);
        if (y > o) our_wins++;
        else if (o > y) their_wins++;
    }

    if (our_wins > their_wins) return "We won";
    if (our_wins < their_wins) return "We lost";
    return "It was a tie";
}


