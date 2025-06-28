// Time Complexity: O(1) - all operations are constant time
// Space Complexity: O(1) - only a fixed number of variables used
public class MilePace {
    public static String milePace(double miles, String timeStr) {
        // Validate miles
        if (miles <= 0) {
            throw new IllegalArgumentException("Miles must be a positive number.");
        }
        // Validate time string format
        if (timeStr == null || !timeStr.contains(":")) {
            throw new IllegalArgumentException("Time string must be in MM:SS format.");
        }
        String[] parts = timeStr.split(":");
        if (parts.length != 2) {
            throw new IllegalArgumentException("Time string must be in MM:SS format.");
        }
        int minutes, seconds;
        try {
            minutes = Integer.parseInt(parts[0]);
            seconds = Integer.parseInt(parts[1]);
        } catch (NumberFormatException e) {
            throw new IllegalArgumentException("Time string must contain valid integers in MM:SS format.");
        }
        // Convert minutes and seconds to total seconds
        int totalSeconds = minutes * 60 + seconds;

        // Calculate average seconds per mile
        double paceSeconds = totalSeconds / miles;

        // Convert pace in seconds back to minutes and seconds
        int paceMinutes = (int) (paceSeconds / 60);
        int paceRemainingSeconds = (int) Math.round(paceSeconds % 60);

        // If rounding results in 60 seconds, increment minutes and reset seconds
        if (paceRemainingSeconds == 60) {
            paceMinutes += 1;
            paceRemainingSeconds = 0;
        }

        // Format the result as MM:SS string
        return String.format("%02d:%02d", paceMinutes, paceRemainingSeconds);
    }
    // This function calculates the average pace per mile given the total distance (in miles)
    // and the total time (in MM:SS format). It returns the pace as a string in MM:SS format.
}
