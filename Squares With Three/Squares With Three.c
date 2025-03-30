// Time Complexity: O(n)
// Space Complexity: O(1)

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

// Function to count numbers whose squares contain the given digit
int squares_with_digit(int n, char digit) {
    // Validate that n is positive
    if (n <= 0) {
        printf("Input must be a positive integer.\n");
        return -1;
    }
    // Validate that digit is a numeric character
    if (digit < '0' || digit > '9') {
        printf("Digit must be a numeric character.\n");
        return -1;
    }
    int count = 0; // Initialize counter
    char buffer[32]; // Buffer to hold string representation of square
    for (int i = 1; i <= n; i++) { // Loop from 1 to n
        sprintf(buffer, "%d", i * i); // Convert square to string
        if (strchr(buffer, digit)) { // Check if digit is present in the string
            count++; // Increment counter if condition is met
        }
    }
    return count; // Return the final count
}

int main() {
    int n = 10; // Example input value
    char digit = '3'; // Example digit to search for
    // Print the result
    printf("Count: %d\n", squares_with_digit(n, digit));
    return 0;
}
// This program counts numbers from 1 to n whose squares contain the digit. It uses string conversion and character search to analyze digit patterns in C.
