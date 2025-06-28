// Time Complexity: O(1) - all operations are constant time
// Space Complexity: O(1) - only a fixed number of variables used
String milePace(double miles, String timeStr) {
  // Validate miles
  if (miles <= 0) {
    throw ArgumentError('Miles must be a positive number.');
  }
  // Validate time string format
  if (timeStr is! String || !timeStr.contains(":")) {
    throw ArgumentError('Time string must be in MM:SS format.');
  }
  var parts = timeStr.split(":");
  if (parts.length != 2) {
    throw ArgumentError('Time string must be in MM:SS format.');
  }
  int minutes, seconds;
  try {
    minutes = int.parse(parts[0]);
    seconds = int.parse(parts[1]);
  } catch (e) {
    throw ArgumentError('Time string must contain valid integers in MM:SS format.');
  }
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
