pub fn assert_is_close(actual: f64, expected: f64, tolerance: f64) {
  assert!(
    (actual - expected).abs() < tolerance,
    "actual {} is not within {} of expected {}",
    actual, tolerance, expected
  );
}

use mock_instant::thread_local::MockClock;
use std::time::Duration;

pub fn advance_clock(seconds: f64) {
  MockClock::advance(Duration::from_secs_f64(seconds));
}
