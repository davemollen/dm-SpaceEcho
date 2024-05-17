use crate::FloatExt;

struct Ramp {
  prev: f32,
  index: u32,
  step_size: f32,
}

pub struct RampSlide {
  slide_up_factor: f32,
  ramp_down_factor: f32,
  ramp_down: f32,
  z: f32,
  ramp: Ramp,
}

impl RampSlide {
  pub fn new(sample_rate: f32, slide_up: f32, ramp_down: f32) -> Self {
    Self {
      z: 1.,
      slide_up_factor: slide_up.mstosamps(sample_rate).recip(),
      ramp_down_factor: ramp_down.mstosamps(sample_rate).recip(),
      ramp_down,
      ramp: Ramp {
        prev: 1.,
        index: 0,
        step_size: 0.,
      },
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    if input.is_equal_to(self.z) {
      input
    } else {
      let difference = input - self.z;
      if input > self.z {
        self.slide_up(difference)
      } else {
        self.ramp_down(input, difference)
      }
    }
  }

  fn slide_up(&mut self, difference: f32) -> f32 {
    self.z += difference * self.slide_up_factor;
    self.z
  }

  fn ramp_down(&mut self, input: f32, difference: f32) -> f32 {
    if input != self.ramp.prev {
      self.ramp.index = self.ramp_down as u32;
      self.ramp.step_size = difference * self.ramp_down_factor;
      self.ramp.prev = input;
    }

    if self.ramp.index > 0 {
      self.ramp.index -= 1;
      self.z += self.ramp.step_size;
    }
    self.z
  }
}

#[cfg(test)]
mod tests {
  use super::RampSlide;

  #[test]
  fn should_ramp_down_in_time() {
    let slide_up = 2.0;
    let ramp_down = 4.0;
    let mut ramp_slide = RampSlide::new(1000., slide_up, ramp_down);

    assert_eq!(ramp_slide.process(1.), 1.0);
    assert_eq!(ramp_slide.process(1.), 1.0);
    assert_eq!(ramp_slide.process(0.5), 0.875);
    assert_eq!(ramp_slide.process(0.5), 0.75);
    assert_eq!(ramp_slide.process(0.5), 0.625);
    assert_eq!(ramp_slide.process(0.5), 0.5);
    assert_eq!(ramp_slide.process(0.5), 0.5);
    assert_eq!(ramp_slide.process(0.5), 0.5);
    assert_eq!(ramp_slide.process(0.), 0.375);
    assert_eq!(
      ramp_slide.process(0.25),
      0.375 - ((0.375 - 0.25) / ramp_down)
    );
  }
}
