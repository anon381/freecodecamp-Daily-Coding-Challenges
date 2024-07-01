#include <stdio.h>

void fibonacci_sequence(long long start1, long long start2, int length) {
    if (length == 0) return;
    if (length == 1) {
        printf("%lld\n", start1);
        return;
    }
    if (length == 2) {
        printf("%lld %lld\n", start1, start2);
        return;
    }

    long long a = start1, b = start2;
    printf("%lld %lld", a, b);
    for (int i = 2; i < length; i++) {
        long long next = a + b;
        printf(" %lld", next);
        a = b;
        b = next;
    }
    printf("\n");
}

int main() {
    fibonacci_sequence(0, 1, 10);
    fibonacci_sequence(123456789LL, 987654321LL, 5);
    return 0;
}
