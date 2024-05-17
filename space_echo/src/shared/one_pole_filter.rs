use crate::shared::float_ext::FloatExt;
use std::f32::consts::TAU;

pub struct OnePoleFilter {
  t: f32,
  z: f32,
}

impl OnePoleFilter {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      t: sample_rate.recip() * -TAU,
      z: 0.,
    }
  }

  pub fn process(&mut self, input: f32, cutoff_freq: f32) -> f32 {
    let b1 = (cutoff_freq * self.t).fast_exp();
    let a0 = 1.0 - b1;
    self.z = input * a0 + self.z * b1;
    self.z
  }
}
