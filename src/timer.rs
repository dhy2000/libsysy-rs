use std::{time::{Duration, Instant}, fmt};

use ctor::dtor;

struct Timespan {
    line_start: usize,
    line_end: usize,
    duration: Duration,
}

fn duration_fmt(d: &Duration) -> String {
    let mut t = d.as_secs();
    let ss = t % 60;
    t /= 60;
    let mm = t % 60;
    t /= 60;
    let hh = t;
    let micros = d.subsec_micros();
    format!("{}H-{}M-{}S-{}us", hh, mm, ss, micros)
}

impl fmt::Display for Timespan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Timer@{:04}-{:04}: {}", self.line_start, self.line_end, duration_fmt(&self.duration))
    }
}

struct Timer {
    line_start: usize,
    instant: Instant,
}

impl Timer {
    fn new(line_start: usize) -> Self {
        Self { line_start, instant: Instant::now() }
    }

    fn end(&self, line_end: usize) -> Timespan {
        Timespan { line_start: self.line_start, line_end, duration: self.instant.elapsed() }
    }
}

static mut DURATIONS: Vec<Timespan> = Vec::new();
static mut CURRENT: Option<Timer> = None;

pub fn start_timer(line_number: usize) {
    unsafe {
        CURRENT = Some(Timer::new(line_number));
    }
}

pub fn stop_timer(line_number: usize) {
    unsafe {
        if let Some(current) = &CURRENT {
            DURATIONS.push(current.end(line_number));
        }
    }
}

#[dtor]
fn finalize_timers() {
    let total: Duration;
    let timers: String;
    unsafe {
        total = DURATIONS.iter().map(|x| x.duration).reduce(|a, b| a + b).unwrap_or(Duration::ZERO);
        timers = DURATIONS.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n");
    }
    eprintln!("{}", timers);
    eprintln!("TOTAL: {}", duration_fmt(&total));
}
