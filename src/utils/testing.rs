use std::net::SocketAddr;
use std::time::Duration;
use mock_instant::thread_local::MockClock;

pub fn assert_is_close(actual: f64, expected: f64, tolerance: f64) {
  assert!(
    (actual - expected).abs() < tolerance,
    "actual {} is not within {} of expected {}",
    actual, tolerance, expected
  );
}

pub fn addr_from(addr: &str) -> SocketAddr { addr.parse().unwrap() }
pub fn addr() -> SocketAddr { addr_from("127.1.1.11:3322") }

pub fn addrs_from(addrs: &str) -> Vec<SocketAddr> {
  return addrs.split(", ")
    .map(|a| a.parse().unwrap() )
    .collect();
}
pub fn addrs() -> Vec<SocketAddr> {
  addrs_from("127.1.1.11:3322, 127.1.1.12:3322, 127.1.1.13:3322")
}

pub fn advance_clock(seconds: f64) {
  MockClock::advance(Duration::from_secs_f64(seconds));
}
