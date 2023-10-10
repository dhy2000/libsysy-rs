use libc::{getchar, printf, putchar, scanf};
use std::io::{self, Write};

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
    unsafe { getchar() }
}

pub fn putch(c: i32) {
    unsafe {
        putchar(c);
    }
}

#[allow(dead_code)]
fn put<T>(o: T)
where
    T: ToString,
{
    let s = o.to_string() + "\0";
    unsafe {
        printf("%s\0".as_ptr() as *const i8, s.as_ptr());
    }
}

#[allow(dead_code)]
fn putln<T>(o: T)
where
    T: ToString,
{
    let s = o.to_string() + "\0";
    unsafe {
        printf("%s\n\0".as_ptr() as *const i8, s.as_ptr());
    }
}

pub fn getarray(a: *mut i32) -> i32 {
    let n = getint();
    let data = (0..n).map(|_| getint()).collect::<Vec<i32>>();
    unsafe {
        std::ptr::copy(data.as_ptr(), a, n as usize);
    }
    n
}

pub fn getfarray(a: *mut f32) -> i32 {
    let n = getint();
    let data = (0..n).map(|_| getfloat()).collect::<Vec<f32>>();
    unsafe {
        std::ptr::copy(data.as_ptr(), a, n as usize);
    }
    n
}

pub fn putarray(n: i32, a: *const i32) {
    let n = n as usize;
    let data;
    unsafe {
        data = std::slice::from_raw_parts(a, n).to_vec();
    }
    put(format!("{}: ", n));
    putln(data
        .into_iter()
        .map(|x| x.to_string())
        .reduce(|acc, t| acc + " " + &t)
        .unwrap_or(String::new()));
}

pub fn putfarray(n: i32, a: *const f32) {
    let n = n as usize;
    let data;
    unsafe {
        data = std::slice::from_raw_parts(a, n).to_vec();
    }
    put(format!("{}:", n));
    for data in data.into_iter() {
        putch(32);
        putfloat(data);
    }
    putch(10);
}
