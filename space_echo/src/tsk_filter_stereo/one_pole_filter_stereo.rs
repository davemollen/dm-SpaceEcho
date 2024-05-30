use std::f32::consts::TAU;

use super::FilterType;

pub struct OnePoleFilterStereo {
  t: f32,
  z: (f32, f32),
  prev_cutoff_freq: f32,
  b1: f32,
}

impl OnePoleFilterStereo {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      t: sample_rate.recip() * -TAU,
      z: (0., 0.),
      prev_cutoff_freq: 0.,
      b1: 0.,
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
    if cutoff_freq != self.prev_cutoff_freq {
      self.b1 = (cutoff_freq * self.t).exp();
      self.prev_cutoff_freq = cutoff_freq;
    }

    let a0 = 1.0 - self.b1;
    self.z = (
      input.0 * a0 + self.z.0 * self.b1,
      input.1 * a0 + self.z.1 * self.b1,
    );
    self.z
  }
}
