#include <stdio.h>
#include <ctype.h>
#include <string.h>

int is_pangram(const char *sentence, const char *letters) {
    int sentenceSet[26] = {0};
    int givenSet[26] = {0};

    for (int i = 0; letters[i]; i++) {
        if (isalpha(letters[i])) {
            givenSet[tolower(letters[i]) - 'a'] = 1;
        }
    }

    for (int i = 0; sentence[i]; i++) {
        if (isalpha(sentence[i])) {
            sentenceSet[tolower(sentence[i]) - 'a'] = 1;
        }
    }

    for (int i = 0; i < 26; i++) {
        if (sentenceSet[i] != givenSet[i]) {
            return 0; 
        }
    }

    return 1; 
}

