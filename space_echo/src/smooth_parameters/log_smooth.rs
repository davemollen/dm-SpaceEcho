pub struct LogSmooth {
  sample_rate: f32,
  z: f32,
}

impl LogSmooth {
  pub fn new(sample_rate: f32, initial_value: f32) -> Self {
    Self {
      sample_rate,
      z: initial_value,
    }
  }

  pub fn process(&mut self, input: f32, factor: f32) -> f32 {
    let difference = input - self.z;
    if difference.is_subnormal() {
      input
    } else {
      let ad = 0.693147 / (factor * self.sample_rate).max(1.);
      self.z = difference * ad + self.z;
      self.z
    }
  }
}
