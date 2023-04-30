use std::f32::consts::PI;

pub struct OnePoleFilterStereo {
  sample_rate: f32,
  z: (f32, f32),
}

impl OnePoleFilterStereo {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_rate,
      z: (0., 0.),
    }
  }

  fn z_is_subnormal(&self, input: (f32, f32)) -> bool {
    (input.0 - self.z.0).abs().is_subnormal() && (input.1 - self.z.1).abs().is_subnormal()
  }

  fn apply_filter(&mut self, input: (f32, f32), freq: f32) -> (f32, f32) {
    let b1 = (-2.0 * PI * freq / self.sample_rate).exp();
    let a0 = 1.0 - b1;
    self.z = (input.0 * a0 + self.z.0 * b1, input.1 * a0 + self.z.1 * b1);
    self.z
  }

  pub fn run(&mut self, input: (f32, f32), cutoff_freq: f32) -> (f32, f32) {
    if self.z_is_subnormal(input) {
      input
    } else {
      self.apply_filter(input, cutoff_freq)
    }
  }
}
