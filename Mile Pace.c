// Time Complexity: O(1) - all operations are constant time
// Space Complexity: O(1) - only a fixed number of variables used
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

char* mile_pace(double miles, const char* time_str) {
    // Validate miles
    if (miles <= 0) {
        fprintf(stderr, "Miles must be a positive number.\n");
        return NULL;
    }
    // Validate time string format
    if (!time_str || !strchr(time_str, ':')) {
        fprintf(stderr, "Time string must be in MM:SS format.\n");
        return NULL;
    }
    int minutes, seconds;
    if (sscanf(time_str, "%d:%d", &minutes, &seconds) != 2) {
        fprintf(stderr, "Time string must contain valid integers in MM:SS format.\n");
        return NULL;
    }
    // Convert minutes and seconds to total seconds
    int total_seconds = minutes * 60 + seconds;

    // Calculate average seconds per mile
    double pace_seconds = total_seconds / miles;

    // Convert pace in seconds back to minutes and seconds
    int pace_minutes = (int)(pace_seconds / 60);
    int pace_remaining_seconds = (int)round(fmod(pace_seconds, 60));

    // If rounding results in 60 seconds, increment minutes and reset seconds
    if (pace_remaining_seconds == 60) {
        pace_minutes += 1;
        pace_remaining_seconds = 0;
    }

    // Format the result as MM:SS string
    char* result = malloc(6);
    snprintf(result, 6, "%02d:%02d", pace_minutes, pace_remaining_seconds);
    return result;
}

// This function calculates the average pace per mile given the total distance (in miles)
// and the total time (in MM:SS format). It returns the pace as a string in MM:SS format.
