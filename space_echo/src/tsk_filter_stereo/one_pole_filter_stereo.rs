use crate::FloatExt;
use std::f32::consts::TAU;

use super::FilterType;

pub struct OnePoleFilterStereo {
  sample_period: f32,
  z: (f32, f32),
}

impl OnePoleFilterStereo {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_period: sample_rate.recip(),
      z: (0., 0.),
    }
  }

  pub fn process(&mut self, input: (f32, f32), freq: f32, filter_type: FilterType) -> (f32, f32) {
    match filter_type {
      FilterType::Lowpass => self.apply_filter(input, freq),
      FilterType::Highpass => {
        let filter_output = self.apply_filter(input, freq);
        (input.0 - filter_output.0, input.1 - filter_output.1)
      }
    }
  }

  fn apply_filter(&mut self, input: (f32, f32), cutoff_freq: f32) -> (f32, f32) {
    let b1 = (-TAU * cutoff_freq * self.sample_period).fast_exp();
    let a0 = 1.0 - b1;
    self.z = (input.0 * a0 + self.z.0 * b1, input.1 * a0 + self.z.1 * b1);
    self.z
  }
}
