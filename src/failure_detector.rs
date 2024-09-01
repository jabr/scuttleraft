#[cfg(not(test))]
use super::utils::Touch;

#[cfg(test)]
use super::utils::testing::ManualTouch as Touch;

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

  fn assert_is_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
      (actual - expected).abs() < tolerance,
      "actual {} is not within {} of expected {}",
      actual, tolerance, expected
    );
  }

  #[test]
  fn test_with_no_variance_phi_increases_since_the_last_update() {
    let mut d = FailureDetector::default();
    assert_eq!(d.phi(), 0.0);

    d.touch.adjust(0.5);
    assert_eq!(d.phi(), 0.5);

    d.touch.adjust(0.5);
    assert_eq!(d.phi(), 1.0);

    d.touch.adjust(1.0);
    assert_eq!(d.phi(), 2.0);
    assert_eq!(d.failed(), false);

    d.touch.adjust(10.0);
    assert_eq!(d.phi(), 12.0);
    assert_eq!(d.failed(), true);
  }

  #[test]
  fn test_with_some_variance_phi_increases_more_slowly() {
    let mut d = FailureDetector::default();
    d.touch.adjust(2.0);
    d.update();
    assert_is_close(d.variance(), 0.0899999, 1e-7);
    assert_eq!(d.phi(), 0.0);

    d.touch.adjust(0.5);
    assert_is_close(d.phi(), 0.2941176, 1e-7);

    d.touch.adjust(0.5);
    assert_is_close(d.phi(), 0.5882353, 1e-7);

    d.touch.adjust(1.0);
    assert_is_close(d.phi(), 1.1764706, 1e-7);

    d.touch.adjust(10.0);
    assert_is_close(d.phi(), 7.0588235, 1e-7);
  }

  #[test]
  fn test_update_interval_consistency_affects_variance() {
    let mut d = FailureDetector::default();

    d.touch.adjust(0.01);
    d.update();
    assert_is_close(d.variance(), 0.0882089, 1e-7);
    assert_is_close(d.mean, 0.901, 1e-7);

    d.touch.adjust(1.5);
    d.update();
    assert_is_close(d.variance(), 0.1116801, 1e-7);
    assert_is_close(d.mean, 0.9609, 1e-7);

    d.touch.adjust(10.0);
    d.update();
    assert_is_close(d.variance(), 7.4539917, 1e-7);
    assert_is_close(d.mean, 1.8648099, 1e-7);

    d.touch.adjust(0.2);
    d.update();
    assert_is_close(d.variance(), 6.9580358, 1e-7);
    assert_is_close(d.mean, 1.698329, 1e-7);

    d.touch.adjust(1.0);
    d.update();
    assert_is_close(d.variance(), 6.3061220, 1e-7);
    assert_is_close(d.mean, 1.6284961, 1e-7);

    d.touch.adjust(1.0);
    d.update();
    assert_is_close(d.variance(), 5.7110604, 1e-7);
    assert_is_close(d.mean, 1.5656464, 1e-7);

    d.touch.adjust(1.0);
    d.update();
    assert_is_close(d.variance(), 5.1687504, 1e-7);
    assert_is_close(d.mean, 1.5090818, 1e-7);

    d.touch.adjust(1.0);
    d.update();
    assert_is_close(d.variance(), 4.6752002, 1e-7);
    assert_is_close(d.mean, 1.4581736, 1e-7);

    d.touch.adjust(1.4582);
    d.update();
    assert_is_close(d.variance(), 4.2076801, 1e-7);
    assert_is_close(d.mean, 1.4581762, 1e-7);

    d.touch.adjust(1.4582);
    d.update();
    assert_is_close(d.variance(), 3.7869121, 1e-7);
    assert_is_close(d.mean, 1.4581786, 1e-7);
  }
}
