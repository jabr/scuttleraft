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

pub fn rng(seed: Option<u128>) -> oorandom::Rand64 {
  oorandom::Rand64::new(match seed {
    Some(s) => { s }
    None => { generate_seed() }
  })
}

static mut RNG: Option<oorandom::Rand64> = None;
pub fn Rng() -> oorandom::Rand64 {
  unsafe {
  match RNG {
    Some(r) => { r }
    None => {
      let r = rng(None);
      RNG = Some(r);
      r
    }
  }
}
}

// Fisher–Yates shuffle.
// * Note: this modifies the input array.
pub fn shuffle<T>(rng: &mut oorandom::Rand64, array: &mut Vec<T>, max: usize) {
  let len = array.len();
  let m = usize::min(max, len - 2);
  for i in 0..m {
    let j = rng.rand_range(i as u64 .. len as u64) as usize;
    array.swap(i, j);
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
