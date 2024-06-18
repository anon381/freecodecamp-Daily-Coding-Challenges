#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

void find_missing_numbers(const int *arr, int len) {
    if (len == 0) {
        printf("[]\n");
        return;
    }
    int n = arr[0];
    for (int i = 1; i < len; i++) if (arr[i] > n) n = arr[i];
    bool *present = calloc(n + 1, sizeof(bool));
    for (int i = 0; i < len; i++) present[arr[i]] = true;
    printf("[");
    int first = 1;
    for (int i = 1; i <= n; i++) {
        if (!present[i]) {
            if (!first) printf(", ");
            printf("%d", i);
            first = 0;
        }
    }
    printf("]\n");
    free(present);
}

int main() {
    int arr[] = {1, 2, 4, 6, 6, 3, 7, 8};
    find_missing_numbers(arr, 8); // Output: [5]
    int arr2[] = {};
    find_missing_numbers(arr2, 0); // Output: []
    return 0;
}
