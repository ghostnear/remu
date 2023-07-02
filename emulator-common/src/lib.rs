mod timers;
pub use timers::*;

mod maths;
pub use maths::*;

#[inline]
pub fn sleep_seconds_f64(seconds: f64) {
    std::thread::sleep(std::time::Duration::from_secs_f64(seconds));
}
