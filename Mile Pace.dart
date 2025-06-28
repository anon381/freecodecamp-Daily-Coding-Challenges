// Time Complexity: O(1) - all operations are constant time
// Space Complexity: O(1) - only a fixed number of variables used
String milePace(double miles, String timeStr) {
  // Split the time string into minutes and seconds
  var parts = timeStr.split(":");
  int minutes = int.parse(parts[0]);
  int seconds = int.parse(parts[1]);
  // Convert minutes and seconds to total seconds
  int totalSeconds = minutes * 60 + seconds;

  // Calculate average seconds per mile
  double paceSeconds = totalSeconds / miles;

  // Convert pace in seconds back to minutes and seconds
  int paceMinutes = (paceSeconds ~/ 60);
  int paceRemainingSeconds = (paceSeconds % 60).round();

  // If rounding results in 60 seconds, increment minutes and reset seconds
  if (paceRemainingSeconds == 60) {
    paceMinutes += 1;
    paceRemainingSeconds = 0;
  }

  // Format the result as MM:SS string
  return "${paceMinutes.toString().padLeft(2, '0')}:${paceRemainingSeconds.toString().padLeft(2, '0')}";
}

// This function calculates the average pace per mile given the total distance (in miles)
// and the total time (in MM:SS format). It returns the pace as a string in MM:SS format.
