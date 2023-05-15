pub struct SlewRelease {
  sample_rate: f32,
  z: f32,
}

impl SlewRelease {
  pub fn new(sample_rate: f32) -> Self {
    Self { sample_rate, z: 0. }
  }

  fn mstosamps(&self, time: f32) -> f32 {
    time * 0.001 * self.sample_rate
  }

  fn slew_factor(&self, release_time: f32) -> f32 {
    (self.mstosamps(release_time) + 1.).recip()
  }

  pub fn run(&mut self, input: f32, release_time: f32) -> f32 {
    self.z += (input - self.z) * self.slew_factor(release_time);
    self.z = self.z.min(input);
    self.z
  }
}
