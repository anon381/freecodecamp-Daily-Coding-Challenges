// Space Complexity: O(1)
// Time Complexity: O(n)
// Returns the strength value of a character (a-z, A-Z, 0-9)
int charStrength(String c) {
  if (c.length != 1) return 0;
  var code = c.codeUnitAt(0);
  if (code >= 'a'.codeUnitAt(0) && code <= 'z'.codeUnitAt(0)) {
    return code - 'a'.codeUnitAt(0) + 1;
  } else if (code >= 'A'.codeUnitAt(0) && code <= 'Z'.codeUnitAt(0)) {
    return code - 'A'.codeUnitAt(0) + 27;
  } else if (code >= '0'.codeUnitAt(0) && code <= '9'.codeUnitAt(0)) {
    return code - '0'.codeUnitAt(0);
  }
  return 0;
}

// Simulates a battle between two armies and returns the result
String battle(String your, String opp) {
  if (your.length > opp.length) return "Opponent retreated";
  if (your.length < opp.length) return "We retreated";

  int ourWins = 0, theirWins = 0;
  for (int i = 0; i < your.length; i++) {
    int y = charStrength(your[i]);
    int o = charStrength(opp[i]);
    if (y > o) {
      ourWins++;
    } else if (o > y) {
      theirWins++;
    }
  }

  if (ourWins > theirWins) return "We won";
  if (ourWins < theirWins) return "We lost";
  return "It was a tie";
}


