#include "stdio.h"
#include "stdint.h"

int32_t add(int32_t x, int32_t y);

int main(void) {
    int32_t x = add(62, 30);
    printf("%d\n", (int)x);
    return 0;
}
