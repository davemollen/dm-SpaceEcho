use crate::FloatExt;

pub struct Slide {
  slide_up: f32,
  slide_down: f32,
  z: f32,
}

impl Slide {
  pub fn new(sample_rate: f32, slide_up: f32, slide_down: f32) -> Self {
    Self {
      slide_up: slide_up.mstosamps(sample_rate).recip(),
      slide_down: slide_down.mstosamps(sample_rate).recip(),
      z: 1.,
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let difference = input - self.z;
    if difference.abs() <= f32::EPSILON {
      input
    } else {
      self.z += difference
        * if input > self.z {
          self.slide_up
        } else {
          self.slide_down
        };
      self.z
    }
  }
}
