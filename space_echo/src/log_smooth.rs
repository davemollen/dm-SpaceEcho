pub struct LogSmooth {
  sample_rate: f32,
  z: f32,
}

impl LogSmooth {
  pub fn new(sample_rate: f32) -> Self {
    Self { sample_rate, z: 0. }
  }

  pub fn run(&mut self, input: f32, factor: f32) -> f32 {
    let ad = 0.693147 / (factor * self.sample_rate).max(1.);
    self.z = ((input - self.z) * ad) + self.z;
    self.z
  }
}
