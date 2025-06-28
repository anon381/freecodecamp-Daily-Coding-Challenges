# Time Complexity: O(1) - all operations are constant time
# Space Complexity: O(1) - only a fixed number of variables used
def mile_pace(miles: float, time_str: str) -> str:
	# Validate miles
	if not isinstance(miles, (int, float)) or miles <= 0:
		raise ValueError("Miles must be a positive number.")
	# Validate time string format
	if not isinstance(time_str, str) or ":" not in time_str:
		raise ValueError("Time string must be in MM:SS format.")
	try:
		minutes, seconds = map(int, time_str.split(":"))
	except Exception:
		raise ValueError("Time string must contain valid integers in MM:SS format.")
	# Convert minutes and seconds to total seconds
	total_seconds = minutes * 60 + seconds

	# Calculate average seconds per mile
	pace_seconds = total_seconds / miles

	# Convert pace in seconds back to minutes and seconds
	pace_minutes = int(pace_seconds // 60)
	pace_remaining_seconds = int(round(pace_seconds % 60))

	# If rounding results in 60 seconds, increment minutes and reset seconds
	if pace_remaining_seconds == 60:
		pace_minutes += 1
		pace_remaining_seconds = 0

	# Format the result as MM:SS string
	return f"{pace_minutes:02d}:{pace_remaining_seconds:02d}"

# This function calculates the average pace per mile given the total distance (in miles)
# and the total time (in MM:SS format). It returns the pace as a string in MM:SS format.
