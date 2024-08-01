use std::f32::consts::TAU;

pub struct OnePoleFilter {
  t: f32,
  z: [f32; 4],
  b1: f32,
}

impl OnePoleFilter {
  pub fn new(sample_rate: f32, freq: f32) -> Self {
    let t = sample_rate.recip() * -TAU;
    Self {
      t: sample_rate.recip() * -TAU,
      z: [0.; 4],
      b1: (freq * t).exp(),
    }
  }

  pub fn process(&mut self, input: [f32; 4]) -> [f32; 4] {
    let a0 = 1. - self.b1;
    self.z[0] = input[0] * a0 + self.z[0] * self.b1;
    self.z[1] = input[1] * a0 + self.z[1] * self.b1;
    self.z[2] = input[2] * a0 + self.z[2] * self.b1;
    self.z[3] = input[3] * a0 + self.z[3] * self.b1;

    self.z
  }
}
