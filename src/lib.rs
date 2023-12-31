mod timer;
mod io;

#[no_mangle]
pub extern "C" fn _sysy_starttime(line: i32) {
    timer::start_timer(line as usize);
}

#[no_mangle]
pub extern "C" fn _sysy_stoptime(line: i32) {
    timer::stop_timer(line as usize);
}

#[no_mangle]
pub extern "C" fn getint() -> i32 {
    io::getint()
}

#[no_mangle]
pub extern "C" fn putint(n: i32) {
    io::putint(n);
}

#[no_mangle]
pub extern "C" fn getfloat() -> f32 {
    io::getfloat()
}

#[no_mangle]
pub extern "C" fn putfloat(f: f32) {
    io::putfloat(f);
}

#[no_mangle]
pub extern "C" fn getch() -> i32 {
    io::getch()
}

#[no_mangle]
pub extern "C" fn putch(c: i32) {
    io::putch(c);
}

#[no_mangle]
pub extern "C" fn getarray(a: *mut i32) -> i32 {
    io::getarray(a)
}

#[no_mangle]
pub extern "C" fn putarray(n: i32, a: *const i32) {
    io::putarray(n, a)
}

#[no_mangle]
pub extern "C" fn getfarray(a: *mut f32) -> i32 {
    io::getfarray(a)
}

#[no_mangle]
pub extern "C" fn putfarray(n: i32, a: *const f32) {
    io::putfarray(n, a)
}