use std::f32::consts::TAU;

pub struct Average {
  z: f32,
  b1: f32,
}

impl Average {
  pub fn new(sample_rate: f32, filter_freq: f32) -> Self {
    let t = sample_rate.recip() * -TAU;

    Self {
      z: 0.,
      b1: (filter_freq * t).exp(),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let squared = input * input;
    let filtered = self.filter(squared);
    filtered.sqrt()
  }

  fn filter(&mut self, input: f32) -> f32 {
    let a0 = 1.0 - self.b1;
    self.z = input * a0 + self.z * self.b1;
    self.z
  }
}
