use std::{f32::consts::TAU, simd::f32x4};

#[derive(Clone, Copy)]
pub struct OnePoleFilter {
  t: f32,
  z: f32x4,
  prev_freq: f32,
  b1: f32x4,
}

impl OnePoleFilter {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      t: sample_rate.recip() * -TAU,
      z: f32x4::splat(0.),
      prev_freq: 0.,
      b1: f32x4::splat(0.),
    }
  }

  pub fn process(&mut self, input: f32x4, freq: f32) -> f32x4 {
    if freq != self.prev_freq {
      self.b1 = f32x4::splat((freq * self.t).exp());
      self.prev_freq = freq;
    }

    let a0 = f32x4::splat(1.0) - self.b1;
    self.z = input * a0 + self.z * self.b1;
    self.z
  }
}
