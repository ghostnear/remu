mod timers;
pub use timers::{DeltaTimer, GenericTimer, GenericTimerConfig};

mod maths;
pub use maths::clamp;

#[inline]
pub fn sleep_seconds_f64(seconds: f64) {
    std::thread::sleep(std::time::Duration::from_secs_f64(seconds));
}
