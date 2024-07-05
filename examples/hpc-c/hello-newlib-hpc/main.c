#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <math.h>


int main()
{
    printf("Hello, world! (from newlib)\n");   
    int num[3];
    scanf("%d %d %d", &num[0], &num[1], &num[2]);
    printf("The numbers you entered are: %f %f %d\n\r", 
        cos((double)num[0]), 
        cos((double)num[1]), 
        num[2]
    );
    printf("The numbers you entered are: %f %f %d\n\r", 
        sin((double)num[0]), 
        sin((double)num[1]), 
        num[2]
    );
    printf("The numbers you entered are: %f %f %d\n\r", 
        tan((double)num[0]), 
        tan((double)num[1]), 
        num[2]
    );

    return 0;
}
