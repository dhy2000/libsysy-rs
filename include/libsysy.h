#ifndef _LIB_SYSY_H_
#define _LIB_SYSY_H_

int getint();
int getch();
int getarray(int a[]);
float getfloat();
int getfarray(float a[]);

void putint(int a);
void putch(int a);
void putarray(int n,int a[]);
void putfloat(float a);
void putfarray(int n, float a[]);

// void putf(char a[], ...);

/* Timing function implementation */
#define starttime() _sysy_starttime(__LINE__)
#define stoptime()  _sysy_stoptime(__LINE__)
void _sysy_starttime(int lineno);
void _sysy_stoptime(int lineno);

#endif