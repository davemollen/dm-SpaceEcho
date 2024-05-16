use crate::float_ext::FloatExt;
use std::f32::consts::PI;

pub struct OnePoleFilter {
  sample_period: f32,
  z: f32,
}

impl OnePoleFilter {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_period: sample_rate.recip(),
      z: 0.,
    }
  }

  pub fn process(&mut self, input: f32, cutoff_freq: f32) -> f32 {
    let b1 = (-2.0 * PI * cutoff_freq * self.sample_period).fast_exp();
    let a0 = 1.0 - b1;
    self.z = input * a0 + self.z * b1;
    self.z
  }

  pub fn process_param(&mut self, input: f32, cutoff_freq: f32) -> f32 {
    if (input - self.z).is_subnormal() {
      input
    } else {
      let b1 = (-2.0 * PI * cutoff_freq * self.sample_period).fast_exp();
      let a0 = 1.0 - b1;
      self.z = input * a0 + self.z * b1;
      self.z
    }
  }
}
