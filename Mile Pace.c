// Time Complexity: O(1) - all operations are constant time
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

char* mile_pace(double miles, const char* time_str) {
    int minutes, seconds;
    sscanf(time_str, "%d:%d", &minutes, &seconds);
    int total_seconds = minutes * 60 + seconds;

    double pace_seconds = total_seconds / miles;
    int pace_minutes = (int)(pace_seconds / 60);
    int pace_remaining_seconds = (int)round(fmod(pace_seconds, 60));

    if (pace_remaining_seconds == 60) {
        pace_minutes += 1;
        pace_remaining_seconds = 0;
    }

    char* result = malloc(6);
    snprintf(result, 6, "%02d:%02d", pace_minutes, pace_remaining_seconds);
    return result;
}
