#include <stdint.h>
#include <string.h>

#include "boot.h"
#include "headsail_bsp.h"

int main() {
    const char *str = "Hello world";

    for (int i = 0; i < strlen(str); i++) {
        putc(str[i]);
    }

    return 0;
}
