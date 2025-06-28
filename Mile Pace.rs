// Time Complexity: O(1) - all operations are constant time
pub fn mile_pace(miles: f64, time_str: &str) -> String {
    // Step 1: convert time string "MM:SS" to total seconds
    let parts: Vec<&str> = time_str.split(':').collect();
    let minutes: i32 = parts[0].parse().unwrap();
    let seconds: i32 = parts[1].parse().unwrap();
    let total_seconds = minutes * 60 + seconds;

    // Step 2: average seconds per mile
    let pace_seconds = total_seconds as f64 / miles;

    // Step 3: convert back to MM:SS
    let mut pace_minutes = (pace_seconds / 60.0).floor() as i32;
    let mut pace_remaining_seconds = (pace_seconds % 60.0).round() as i32;

    // Handle rounding edge case (e.g. 59.9 sec -> 60 sec)
    if pace_remaining_seconds == 60 {
        pace_minutes += 1;
        pace_remaining_seconds = 0;
    }

    format!("{:02}:{:02}", pace_minutes, pace_remaining_seconds)
}
