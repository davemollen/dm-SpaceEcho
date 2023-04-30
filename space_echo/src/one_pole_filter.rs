use std::f32::consts::PI;

pub struct OnePoleFilter {
  sample_rate: f32,
  z: f32,
}

impl OnePoleFilter {
  pub fn new(sample_rate: f32) -> Self {
    Self { sample_rate, z: 0. }
  }

  fn apply_filter(&mut self, input: f32, freq: f32) -> f32 {
    let b1 = (-2.0 * PI * freq / self.sample_rate).exp();
    let a0 = 1.0 - b1;
    self.z = input * a0 + self.z * b1;
    self.z
  }

  pub fn run(&mut self, input: f32, cutoff_freq: f32) -> f32 {
    if (input - self.z).abs().is_subnormal() {
      input
    } else {
      self.apply_filter(input, cutoff_freq)
    }
  }
}
