use std::time::{Duration, Instant};

pub fn measure_time<T, F>(f: F) -> (T, Duration)
where
    F: FnOnce() -> T,
{
    let start_time = Instant::now();
    let result = f();
    let duration = start_time.elapsed();
    (result, duration)
}
