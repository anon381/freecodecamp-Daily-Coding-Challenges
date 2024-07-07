#include <stdio.h>
#include <string.h>
#include <ctype.h>
// Time Complexity: O(n), where n is the length of the input string
// Space Complexity: O(1), only a few variables are used
#include <string.h>
#include <ctype.h>

int is_valid_number(const char *s, int base) {
    for (int i = 0; s[i]; i++) {
        char c = toupper(s[i]);
        int val;
        if (isdigit(c)) {
            val = c - '0';
        } else if (c >= 'A' && c <= 'Z') {
            val = 10 + (c - 'A');
        } else {
            return 0; // invalid char
        }
        if (val >= base) return 0;
    }
    return 1;
}

int main() {
    printf("%d\n", is_valid_number("10101", 2));   // 1
    printf("%d\n", is_valid_number("10201", 2));   // 0
    printf("%d\n", is_valid_number("ABC", 16));    // 1
    printf("%d\n", is_valid_number("5G3F8F", 16)); // 0
    printf("%d\n", is_valid_number("5G3F8F", 17)); // 1
    return 0;
}
