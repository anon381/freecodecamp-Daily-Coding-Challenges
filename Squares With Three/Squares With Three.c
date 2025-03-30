// Time Complexity: O(n)
// Space Complexity: O(1)

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int squares_with_digit(int n, char digit) {
    if (n <= 0) {
        printf("Input must be a positive integer.\n");
        return -1;
    }
    if (digit < '0' || digit > '9') {
        printf("Digit must be a numeric character.\n");
        return -1;
    }
    int count = 0;
    char buffer[32];
    for (int i = 1; i <= n; i++) {
        sprintf(buffer, "%d", i * i);
        if (strchr(buffer, digit)) {
            count++;
        }
    }
    return count;
}

int main() {
    int n = 10;
    char digit = '3';
    printf("Count: %d\n", squares_with_digit(n, digit));
    return 0;
}
    // This program counts numbers from 1 to n whose squares contain the digit '3'. It uses string conversion and character search to analyze digit patterns in C.
