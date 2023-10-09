use std::io::{self, Write};
use libc::{scanf, printf, getchar, putchar};

pub fn getint() -> i32 {
    let mut n: i32 = 0;
    unsafe {
        scanf("%d\0".as_ptr() as *const i8, &mut n as *mut libc::c_int);
    }
    n
}

pub fn putint(n: i32) {
    unsafe {
        printf("%d\0".as_ptr() as *const i8, n);
    }
    io::stdout().flush().unwrap();
}

pub fn getfloat() -> f32 {
    let mut f: f32 = 0.0;
    unsafe {
        scanf("%f\0".as_ptr() as *const i8, &mut f as *mut libc::c_float);
    }
    f
}

pub fn putfloat(f: f32) {
    unsafe {
        printf("%a\0".as_ptr() as *const i8, f as libc::c_double);
    }
    io::stdout().flush().unwrap();
}

pub fn getch() -> i32 {
    unsafe {
        getchar()
    }
}

pub fn putch(c: i32) {
    unsafe {
        putchar(c);
    }
}

