/*
Decimal representation of negative powers of 2
*/

#include <stdio.h>

int main()
{
    const int n = 10;
    int d[n];
    for (int k = 0; k < n; k++)
    {
        printf(".");
        int r = 0;
        for (int i = 0; i <= k - 1; i++)
        {
            r = 10 * r + d[i];
            d[i] = r / 2;
            r = r - 2 * d[i];

            printf("%d", d[i]);
        }
        d[k] = 5;
        printf("5\n");
    }
}
