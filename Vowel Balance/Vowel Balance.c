#include <stdio.h>
#include <string.h>
#include <ctype.h>

int is_vowel(char c) {
    c = tolower(c);
    return (c=='a' || c=='e' || c=='i' || c=='o' || c=='u');
}

int is_balanced(const char *s) {
    int n = strlen(s);
    int half = n / 2;
    int countFirst = 0, countSecond = 0;

    for (int i = 0; i < half; i++) {
        if (is_vowel(s[i])) countFirst++;
    }
    for (int i = n - half; i < n; i++) {
        if (is_vowel(s[i])) countSecond++;
    }

    return countFirst == countSecond;
}

int main() {
    printf("%d\n", is_balanced("racecar")); // 1
    printf("%d\n", is_balanced("Lorem Ipsum")); // 1
    printf("%d\n", is_balanced("Kitty Ipsum")); // 0
    return 0;
}
