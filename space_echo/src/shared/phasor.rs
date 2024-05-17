pub struct Phasor {
  x: f32,
  step_size: f32,
}

impl Phasor {
  pub fn new(sample_rate: f32, freq: f32) -> Self {
    Self {
      x: 0.,
      step_size: sample_rate.recip() * freq,
    }
  }

  pub fn process(&mut self) -> f32 {
    self.x = self.wrap(self.x + self.step_size);
    self.x
  }

  fn wrap(&self, input: f32) -> f32 {
    if input >= 1. {
      input - 1.
    } else {
      input
    }
  }
}
