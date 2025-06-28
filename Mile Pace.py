# Time Complexity: O(1) - all operations are constant time
# Space Complexity: O(1) - only a fixed number of variables used
def mile_pace(miles: float, time_str: str) -> str:
	# Step 1: convert time string "MM:SS" to total seconds
	minutes, seconds = map(int, time_str.split(":"))
	total_seconds = minutes * 60 + seconds

	# Step 2: average seconds per mile
	pace_seconds = total_seconds / miles

	# Step 3: convert back to MM:SS
	pace_minutes = int(pace_seconds // 60)
	pace_remaining_seconds = int(round(pace_seconds % 60))

	# Handle rounding edge case (e.g. 59.9 sec -> 60 sec)
	if pace_remaining_seconds == 60:
		pace_minutes += 1
		pace_remaining_seconds = 0

	return f"{pace_minutes:02d}:{pace_remaining_seconds:02d}"

# This function calculates the average pace per mile given the total distance (in miles)
# and the total time (in MM:SS format). It returns the pace as a string in MM:SS format.
