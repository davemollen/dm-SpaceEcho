use crate::FloatExt;

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
    if input.is_equal_to(self.z) {
      input
    } else {
      let difference = input - self.z;
      let ad = 0.693147 * (factor * self.sample_rate).max(1.).recip();
      self.z += difference * ad;
      self.z
    }
  }
}
