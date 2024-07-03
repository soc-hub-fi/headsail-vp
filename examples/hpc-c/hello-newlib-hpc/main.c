#include <stdint.h>
#include <string.h>
#include <stdio.h>


int main()
{
    printf("Hello, world! (from newlib)\n");   
    char name[50];
    // gets(name);
    scanf("%s", name);
    // printf("Hello %s!\n", name);

    return 0;
}
