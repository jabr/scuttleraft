use super::utils::Touch;

pub struct FailureDetector {
  threshold: f64,
  weight: f64,
  mean: f64,
  squared_interval: f64,
  touch: Touch,
}

impl FailureDetector {
  pub fn new(threshold: f64, weight: f64, interval: f64) -> Self {
    Self {
      threshold,
      weight,
      mean: interval,
      squared_interval: interval * interval,
      touch: Touch::now(),
    }
  }

  pub fn update(&mut self) {
    let interval = self.touch.update();
    let weighted_interval = (1.0 - self.weight) * interval;
    self.mean = self.weight * self.mean + weighted_interval;
    self.squared_interval =
      self.weight * self.squared_interval +
      weighted_interval * interval;
  }

  pub fn variance(&self) -> f64 {
    self.squared_interval - self.mean * self.mean
  }

  fn standard_deviation(&self) -> f64 {
    self.variance().sqrt()
  }

  pub fn phi(&self) -> f64 {
    let interval = self.touch.age();
    interval / (self.mean + 2.0 * self.standard_deviation())
  }

  pub fn failed(&self) -> bool {
    self.phi() > self.threshold
  }
}

impl Default for FailureDetector {
  fn default() -> Self { Self::new(8.0, 0.9, 1.0) }
}

use std::fmt;
impl fmt::Debug for FailureDetector {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("FailtureDetector")
        .field("phi", &self.phi())
        .field("failed", &self.failed())
        .field("last", &self.touch)
        .field("mean", &self.mean)
        .field("variance", &self.variance())
        .finish()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::testing::{assert_is_close, advance_clock};

  #[test]
  fn test_with_no_variance_phi_increases_since_the_last_update() {
    let d = FailureDetector::default();
    assert_eq!(d.phi(), 0.0);

    advance_clock(0.5);
    assert_eq!(d.phi(), 0.5);

    advance_clock(0.5);
    assert_eq!(d.phi(), 1.0);

    advance_clock(1.0);
    assert_eq!(d.phi(), 2.0);
    assert_eq!(d.failed(), false);

    advance_clock(10.0);
    assert_eq!(d.phi(), 12.0);
    assert_eq!(d.failed(), true);
  }

  #[test]
  fn test_with_some_variance_phi_increases_more_slowly() {
    let mut d = FailureDetector::default();
    advance_clock(2.0);
    d.update();
    assert_is_close(d.variance(), 0.0899999, 1e-7);
    assert_eq!(d.phi(), 0.0);

    advance_clock(0.5);
    assert_is_close(d.phi(), 0.2941176, 1e-7);

    advance_clock(0.5);
    assert_is_close(d.phi(), 0.5882353, 1e-7);

    advance_clock(1.0);
    assert_is_close(d.phi(), 1.1764706, 1e-7);

    advance_clock(10.0);
    assert_is_close(d.phi(), 7.0588235, 1e-7);
  }

  #[test]
  fn test_update_interval_consistency_affects_variance() {
    let mut d = FailureDetector::default();

    advance_clock(0.01);
    d.update();
    assert_is_close(d.variance(), 0.0882089, 1e-7);
    assert_is_close(d.mean, 0.901, 1e-7);

    advance_clock(1.5);
    d.update();
    assert_is_close(d.variance(), 0.1116801, 1e-7);
    assert_is_close(d.mean, 0.9609, 1e-7);

    advance_clock(10.0);
    d.update();
    assert_is_close(d.variance(), 7.4539917, 1e-7);
    assert_is_close(d.mean, 1.8648099, 1e-7);

    advance_clock(0.2);
    d.update();
    assert_is_close(d.variance(), 6.9580358, 1e-7);
    assert_is_close(d.mean, 1.698329, 1e-7);

    advance_clock(1.0);
    d.update();
    assert_is_close(d.variance(), 6.3061220, 1e-7);
    assert_is_close(d.mean, 1.6284961, 1e-7);

    advance_clock(1.0);
    d.update();
    assert_is_close(d.variance(), 5.7110604, 1e-7);
    assert_is_close(d.mean, 1.5656464, 1e-7);

    advance_clock(1.0);
    d.update();
    assert_is_close(d.variance(), 5.1687504, 1e-7);
    assert_is_close(d.mean, 1.5090818, 1e-7);

    advance_clock(1.0);
    d.update();
    assert_is_close(d.variance(), 4.6752002, 1e-7);
    assert_is_close(d.mean, 1.4581736, 1e-7);

    advance_clock(1.4582);
    d.update();
    assert_is_close(d.variance(), 4.2076801, 1e-7);
    assert_is_close(d.mean, 1.4581762, 1e-7);

    advance_clock(1.4582);
    d.update();
    assert_is_close(d.variance(), 3.7869121, 1e-7);
    assert_is_close(d.mean, 1.4581786, 1e-7);
  }

  #[test]
  fn test_high_frequency_updates() {
    let mut d = FailureDetector::default();
    assert_eq!(d.phi(), 0.0);

    for _ in 0..100 {
      advance_clock(0.000_000_1);
      d.update();
    }
    assert_is_close(d.phi(), 0.0, 1e-7);

    advance_clock(0.000_000_1);
    assert_is_close(d.phi(), 0.0000097, 1e-7);

    advance_clock(0.1);
    assert_is_close(d.phi(), 9.6767355, 1e-7);
  }

  #[test]
  fn test_low_frequency_updates() {
    let mut d = FailureDetector::default();
    assert_eq!(d.phi(), 0.0);

    for _ in 0..100 {
      advance_clock(1e9);
      d.update();
    }
    assert_is_close(d.phi(), 0.0, 1e-7);

    advance_clock(1e9);
    assert_is_close(d.phi(), 0.9898238, 1e-7);

    advance_clock(0.1);
    assert_is_close(d.phi(), 0.9898238, 1e-7);
  }

  #[test]
  fn test_different_parameters() {
      let mut d1 = FailureDetector::new(5.0, 0.8, 2.0);
      let mut d2 = FailureDetector::new(10.0, 0.95, 0.5);
      assert_eq!(d1.phi(), d2.phi());

      advance_clock(1.0);
      assert_is_close(d1.phi(), 0.5, 1e-7);
      assert_is_close(d2.phi(), 2.0, 1e-7);

      d1.update();
      d2.update();

      advance_clock(1.5);
      assert_is_close(d1.phi(), 0.57692317, 1e-7);
      assert_is_close(d2.phi(), 2.01899213, 1e-7);
  }

  #[test]
  fn test_recovery_from_failure() {
      let mut d = FailureDetector::default();

      // Force failure
      advance_clock(20.0);
      assert!(d.failed());

      // Recover with regular updates
      for _ in 0..10 {
          d.update();
          advance_clock(1.0);
      }

      assert!(!d.failed());
      assert_is_close(d.phi(), 0.1102618, 1e-7);
      assert_is_close(d.mean, 1.7360989, 1e-7);
      assert_is_close(d.variance(), 13.4440380, 1e-7);
  }
}
