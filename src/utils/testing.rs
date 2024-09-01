#[derive(Debug)]
pub struct ManualTouch(f64);

static mut MANUAL_TOUCH_GLOBAL_NOW: f64 = 0.0;

impl ManualTouch {
  pub fn now() -> Self { Self(0.0) }
  pub fn age(&self) -> f64 {
    unsafe { self.0 + MANUAL_TOUCH_GLOBAL_NOW }
  }
  pub fn update(&mut self) -> f64 {
    let elapsed = self.age();
    self.0 = 0.0;
    return elapsed;
  }

  pub fn adjust(&mut self, seconds: f64) { self.0 += seconds; }
  pub fn set_global_now(seconds: f64) {
    unsafe { MANUAL_TOUCH_GLOBAL_NOW = seconds; }
  }
}
