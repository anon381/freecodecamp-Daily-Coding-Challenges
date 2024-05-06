# Time Complexity: O(n), where n is the number of days (typically 7).
#include <stdio.h>
#include <stdbool.h>

bool too_much_screen_time(const int hours[7]) {
    // Rule 1: any single day >= 10
    for (int i = 0; i < 7; i++) {
        if (hours[i] >= 10) return true;
    }
    // Rule 2: average of any 3 consecutive days >= 8
    for (int i = 0; i < 5; i++) {
        int sum = hours[i] + hours[i+1] + hours[i+2];
        if (sum / 3.0 >= 8) return true;
    }
    // Rule 3: weekly average >= 6
    int total = 0;
    for (int i = 0; i < 7; i++) total += hours[i];
    if (total / 7.0 >= 6) return true;
    return false;
}

int main() {
    int week1[7] = {7, 8, 9, 5, 6, 7, 8};
    int week2[7] = {5, 5, 5, 5, 5, 5, 5};
    printf("%d\n", too_much_screen_time(week1)); // Output: 1 (True)
    printf("%d\n", too_much_screen_time(week2)); // Output: 0 (False)
    return 0;
}
