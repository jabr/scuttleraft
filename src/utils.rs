use std::time::Instant;
use oorandom;
use getrandom;

fn generate_seed() -> u128 {
  let mut bytes = [0u8; 16];
  if getrandom::getrandom(&mut bytes).is_err() {
    // as a fallback, use the system time and process id...
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    return nanos ^ std::process::id() as u128;
  }
  bytes.iter().fold(0u128, |a, b| a << 8 | (*b as u128))
}

pub type Rng = oorandom::Rand64;

pub fn rng(seed: Option<u128>) -> Rng {
  Rng::new(seed.unwrap_or_else(|| generate_seed()))
}

pub mod rand {
  // Fisher–Yates shuffle.
  // * Note: this modifies the input array.
  pub fn shuffle<T>(rng: &mut super::Rng, array: &mut Vec<T>, max: usize) {
    let len = array.len();
    let m = usize::min(max, len - 2);
    for i in 0..m {
      let j = rng.rand_range(i as u64 .. len as u64) as usize;
      array.swap(i, j);
    }
  }

  pub fn choose<'a, T>(rng: &mut super::Rng, array: &'a Vec<T>) -> &'a T {
    let index = rng.rand_range(0 .. array.len() as u64) as usize;
    return &array[index];
  }
}

#[derive(Debug)]
pub struct Touch(Instant);

impl Touch {
  pub fn now() -> Self { Self(Instant::now()) }

  pub fn age(&self) -> f64 { self.0.elapsed().as_secs_f64() }

  pub fn update(&mut self) -> f64 {
    let now = Instant::now();
    let elapsed = (now - self.0).as_secs_f64();
    self.0 = now;
    return elapsed;
  }
}

#[derive(Debug)]
pub struct ManualTouch(f64);

impl ManualTouch {
  pub fn now() -> Self { Self(0.0) }
  pub fn age(&self) -> f64 { self.0 }
  pub fn update(&mut self) -> f64 {
    let elapsed = self.0;
    self.0 = 0.0;
    return elapsed;
  }

  pub fn reset(&mut self) { self.0 = 0.0; }
  pub fn adjust(&mut self, seconds: f64) { self.0 += seconds; }
}
