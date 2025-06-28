// Time Complexity: O(1) - all operations are constant time
// Space Complexity: O(1) - only a fixed number of variables used
pub fn mile_pace(miles: f64, time_str: &str) -> String {
    // Split the time string into minutes and seconds
    let parts: Vec<&str> = time_str.split(':').collect();
    let minutes: i32 = parts[0].parse().unwrap();
    let seconds: i32 = parts[1].parse().unwrap();
    // Convert minutes and seconds to total seconds
    let total_seconds = minutes * 60 + seconds;

    // Calculate average seconds per mile
    let pace_seconds = total_seconds as f64 / miles;

    // Convert pace in seconds back to minutes and seconds
    let mut pace_minutes = (pace_seconds / 60.0).floor() as i32;
    let mut pace_remaining_seconds = (pace_seconds % 60.0).round() as i32;

    // If rounding results in 60 seconds, increment minutes and reset seconds
    if pace_remaining_seconds == 60 {
        pace_minutes += 1;
        pace_remaining_seconds = 0;
    }

    // Format the result as MM:SS string
    format!("{:02}:{:02}", pace_minutes, pace_remaining_seconds)
}

// This function calculates the average pace per mile given the total distance (in miles)
// and the total time (in MM:SS format). It returns the pace as a string in MM:SS format.
