#include "libsysy.h"

int main() {
    int a = getint(), b = getint();
    putint(a + b);
    putch(10);
    int i = 0;
    starttime();
    while (i < a) {
        int j = 0;
        while (j < b) {
            j = j + 1;
        }
        i = i + 1;
    }
    stoptime();
    
    float f1 = getfloat(), f2 = getfloat();
    putfloat(f1 + f2);
    putch(10);
    return 0;
}