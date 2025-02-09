

#include <stdio.h>
#include <stdlib.h>

// Time Complexity: O(n1 + n2)
void array_diff(int *arr1, int n1, int *arr2, int n2) {
    int *result = malloc((n1 + n2) * sizeof(int));
    int k = 0;
    for (int i = 0; i < n1; i++) {
        int found = 0;
        for (int j = 0; j < n2; j++) {
            if (arr1[i] == arr2[j]) {
                found = 1;
                break;
            }
        }
        if (!found) result[k++] = arr1[i];
    }
    for (int i = 0; i < n2; i++) {
        int found = 0;
        for (int j = 0; j < n1; j++) {
            if (arr2[i] == arr1[j]) {
                found = 1;
                break;
            }
        }
        if (!found) result[k++] = arr2[i];
    }
    // Sort result
    for (int i = 0; i < k - 1; i++) {
        for (int j = i + 1; j < k; j++) {
            if (result[i] > result[j]) {
                int temp = result[i];
                result[i] = result[j];
                result[j] = temp;
            }
        }
    }
    printf("[");
    for (int i = 0; i < k; i++) {
        printf("%d%s", result[i], i < k - 1 ? ", " : "");
    }
    printf("]\n");
    free(result);
}

int main() {
    int arr1[] = {1, 2, 3};
    int arr2[] = {3, 4, 5};
    array_diff(arr1, 3, arr2, 3);
    return 0;
}
