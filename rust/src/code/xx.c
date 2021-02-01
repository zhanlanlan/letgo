#include <stdio.h>

int main() {
    while (1) {
        int n;
        int a[200];
        scanf("%d", &n);
        if (n == 0) {
            break;
        }

        for (int i = 0; i < n; ++i) {
            scanf("%d", &a[i]);
        }

        // -- 这里你要做处理逻辑

        for (int i=0; i< n; ++i) {
            printf("%d ", a[i]);
        }
        printf("\n");

    }

    return 0;
}