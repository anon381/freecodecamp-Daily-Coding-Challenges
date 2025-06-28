// Time Complexity: O(1) - all operations are constant time
// Space Complexity: O(1) - only a fixed number of variables used
String milePace(double miles, String timeStr) {
  // Step 1: convert time string "MM:SS" to total seconds
  var parts = timeStr.split(":");
  int minutes = int.parse(parts[0]);
  int seconds = int.parse(parts[1]);
  int totalSeconds = minutes * 60 + seconds;

  // Step 2: average seconds per mile
  double paceSeconds = totalSeconds / miles;

  // Step 3: convert back to MM:SS
  int paceMinutes = (paceSeconds ~/ 60);
  int paceRemainingSeconds = (paceSeconds % 60).round();

  // Handle rounding edge case (e.g. 59.9 sec -> 60 sec)
  if (paceRemainingSeconds == 60) {
    paceMinutes += 1;
    paceRemainingSeconds = 0;
  }

  return "${paceMinutes.toString().padLeft(2, '0')}:${paceRemainingSeconds.toString().padLeft(2, '0')}";
}

// This function calculates the average pace per mile given the total distance (in miles)
// and the total time (in MM:SS format). It returns the pace as a string in MM:SS format.
