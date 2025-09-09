// Usage Examples:
// println!("{}", mile_pace(3.0, "24:00")); // Expected: "08:00"
// println!("{}", mile_pace(1.5, "10:30")); // Expected: "07:00"
// Time Complexity: O(1) - all operations are constant time
// Space Complexity: O(1) - only a fixed number of variables used
pub fn mile_pace(miles: f64, time_str: &str) -> String {
    // Validate miles
    if miles <= 0.0 {
        panic!("Miles must be a positive number.");
    }
    // Validate time string format
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 2 {
        panic!("Time string must be in MM:SS format.");
    }
    let minutes: i32 = match parts[0].parse() {
        Ok(m) => m,
        Err(_) => panic!("Time string must contain valid integers in MM:SS format."),
    };
    let seconds: i32 = match parts[1].parse() {
        Ok(s) => s,
        Err(_) => panic!("Time string must contain valid integers in MM:SS format."),
    };
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
