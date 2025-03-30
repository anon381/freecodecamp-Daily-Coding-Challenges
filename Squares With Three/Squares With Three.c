// Time Complexity: O(n)
// Space Complexity: O(1)

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int squares_with_three(int n) {
    int count = 0;
    char buffer[32];
    for (int i = 1; i <= n; i++) {
        sprintf(buffer, "%d", i * i);
        if (strchr(buffer, '3')) {
            count++;
        }
    }
    return count;
}

int main() {
    int n = 10;
    printf("Count: %d\n", squares_with_three(n));
    return 0;
}
    // This program counts numbers from 1 to n whose squares contain the digit '3'. It uses string conversion and character search to analyze digit patterns in C.
