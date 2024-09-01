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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rand_shuffle() {
    let mut rng = rng(Some(42));
    let mut array = vec![ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 ];
    rand::shuffle(&mut rng, &mut array, 7);
    assert_eq!(
      [ 10, 1, 12, 13, 15, 4, 5, 7, 8, 9, 0, 11, 2, 3, 14, 6 ],
      array.as_slice()
    );
  }

  #[test]
  fn test_rand_shuffle_partial() {
    let mut rng = rng(Some(42));
    let mut array = vec![ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 ];
    rand::shuffle(&mut rng, &mut array, 3);
    assert_eq!(
      [ 10, 1, 12, 3, 4, 5, 6, 7, 8, 9, 0, 11, 2, 13, 14, 15 ],
      array.as_slice()
    );
  }

  #[test]
  fn test_rand_choose() {
    let mut rng = rng(Some(42));
    let mut array = vec![ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 ];
    let choice = *rand::choose(&mut rng, &mut array);
    assert_eq!(choice, 10);
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

#[cfg(test)]
pub mod testing;
