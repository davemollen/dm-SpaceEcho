use crate::FloatExt;

pub struct LogSmooth {
  factor: f32,
  z: f32,
}

impl LogSmooth {
  pub fn new(sample_rate: f32, factor: f32) -> Self {
    Self {
      factor: (sample_rate * factor).recip(),
      z: 0.,
    }
  }

  pub fn initialize(&mut self, value: f32) {
    self.z = value;
  }

  pub fn process(&mut self, input: f32) -> f32 {
    if input.is_equal_to(self.z) {
      input
    } else {
      let difference = input - self.z;
      let ad = 0.693147 * self.factor;
      self.z += difference * ad;
      self.z
    }
  }
}
