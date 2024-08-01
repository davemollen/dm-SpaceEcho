use std::{f32::consts::TAU, simd::f32x4};

pub struct OnePoleFilter {
  z: f32x4,
  b1: f32x4,
  a0: f32x4,
}

impl OnePoleFilter {
  pub fn new(sample_rate: f32, freq: f32) -> Self {
    let t = sample_rate.recip() * -TAU;
    let b1 = f32x4::splat((freq * t).exp());

    Self {
      z: f32x4::splat(0.),
      b1,
      a0: f32x4::splat(1.0) - b1,
    }
  }

  pub fn process(&mut self, input: f32x4) -> f32x4 {
    self.z = input * self.a0 + self.z * self.b1;
    self.z
  }
}
