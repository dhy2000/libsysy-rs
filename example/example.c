#include "libsysy.h"

int main() {
    // Int IO
    int a = getint(), b = getint();
    putint(a); putch(32); putint(b); putch(10);
    putint(a + b); putch(10);
    // Float IO
    float f1 = getfloat(), f2 = getfloat();
    putfloat(f1); putch(32); putfloat(f2); putch(10);
    putfloat(f1 + f2); putch(10);
    // Int Array IO
    int arr[16] = {};
    int narr = getarray(arr);
    putint(narr); putch(32); putint(arr[3]); putch(10);
    putarray(narr, arr);
    // Float Array IO
    float farr[16] = {};
    int nfarr = getfarray(farr);
    putint(nfarr); putch(32); putfloat(farr[2]); putch(10);
    putfarray(nfarr, farr);
    // Timing
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
    return 0;
}