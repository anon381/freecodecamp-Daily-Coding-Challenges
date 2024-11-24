// Time Complexity: O(n), where n is the number of characters in the input string.
// Space Complexity: O(n), for storing the words and output.
// C implementation
#include <stdio.h>
#include <string.h>
#include <ctype.h>

void reverse_sentence(const char *s) {
    char words[100][100];
    int word_count = 0, i = 0, j = 0;
    int len = strlen(s);
    while (i < len) {
        while (i < len && isspace(s[i])) i++; // skip spaces
        if (i >= len) break;
        j = 0;
        while (i < len && !isspace(s[i])) {
            words[word_count][j++] = s[i++];
        }
        words[word_count][j] = '\0';
        word_count++;
    }
    for (int k = word_count - 1; k >= 0; k--) {
        printf("%s", words[k]);
        if (k > 0) printf(" ");
    }
    printf("\n");
}

int main() {
    reverse_sentence("hello   world!  this is   c"); // Output: 'c is this world! hello'
    return 0;
}
