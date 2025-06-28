// Time Complexity: O(1) - all operations are constant time
public class MilePace {
    public static String milePace(double miles, String timeStr) {
        // Step 1: convert time string "MM:SS" to total seconds
        String[] parts = timeStr.split(":");
        int minutes = Integer.parseInt(parts[0]);
        int seconds = Integer.parseInt(parts[1]);
        int totalSeconds = minutes * 60 + seconds;

        // Step 2: average seconds per mile
        double paceSeconds = totalSeconds / miles;

        // Step 3: convert back to MM:SS
        int paceMinutes = (int) (paceSeconds / 60);
        int paceRemainingSeconds = (int) Math.round(paceSeconds % 60);

        // Handle rounding edge case (e.g. 59.9 sec -> 60 sec)
        if (paceRemainingSeconds == 60) {
            paceMinutes += 1;
            paceRemainingSeconds = 0;
        }

        return String.format("%02d:%02d", paceMinutes, paceRemainingSeconds);
    }
}
