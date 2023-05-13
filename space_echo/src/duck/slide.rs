pub struct Slide {
  sample_rate: f32,
  z: f32,
}

impl Slide {
  pub fn new(sample_rate: f32) -> Self {
    Self { sample_rate, z: 0. }
  }

  fn mstosamps(&self, time: f32) -> f32 {
    time * 0.001 * self.sample_rate
  }

  pub fn run(&mut self, input: f32, slide_up: f32, slide_down: f32) -> f32 {
    let difference = input - self.z;
    let out = difference
      * self
        .mstosamps(if input > self.z { slide_up } else { slide_down })
        .recip()
      + self.z;
    self.z = out;
    out
  }
}
