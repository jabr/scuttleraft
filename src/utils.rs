use std::time::Instant;

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
