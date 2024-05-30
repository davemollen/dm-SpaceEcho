use crate::shared::float_ext::FloatExt;

pub struct RampSlide {
  slide_up_factor: f32,
  z: f32,
  ramp_prev: f32,
  ramp_index: usize,
  ramp_step_size: f32,
  ramp_time: usize,
  ramp_factor: f32,
}

impl RampSlide {
  pub fn new(sample_rate: f32, slide_up: f32, ramp_down: f32) -> Self {
    let ramp_time = ramp_down.mstosamps(sample_rate);

    Self {
      z: 1.,
      slide_up_factor: slide_up.mstosamps(sample_rate).recip(),
      ramp_prev: 1.,
      ramp_index: 0,
      ramp_step_size: 0.,
      ramp_time: ramp_time as usize - 1,
      ramp_factor: ramp_time.recip(),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    if input.is_equal_to(self.z) {
      input
    } else {
      let difference = input - self.z;
      if difference <= 0. {
        self.ramp_down(input, difference)
      } else {
        self.slide_up(difference)
      }
    }
  }

  fn slide_up(&mut self, difference: f32) -> f32 {
    self.z += difference * self.slide_up_factor;
    self.z
  }

  fn ramp_down(&mut self, input: f32, difference: f32) -> f32 {
    if input != self.ramp_prev {
      let step_size = difference * ((self.ramp_index + 1) as f32).recip();
      if self.ramp_index == 0 || self.ramp_step_size < step_size {
        self.ramp_index = self.ramp_time;
        self.ramp_step_size = difference * self.ramp_factor
      } else {
        self.ramp_step_size = step_size
      }
      self.ramp_prev = input;
    }

    if self.ramp_index == 0 {
      self.z = input;
    }

    if self.ramp_index > 0 {
      self.ramp_index -= 1;
      self.z += self.ramp_step_size;
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

  #[test]
  fn should_ramp_down_twice() {
    let slide_up = 2.0;
    let ramp_down = 4.0;
    let mut ramp_slide = RampSlide::new(1000., slide_up, ramp_down);

    assert_eq!(ramp_slide.process(1.), 1.0);
    assert_eq!(ramp_slide.process(1.), 1.0);
    assert_eq!(ramp_slide.process(0.5), 0.875);
    assert_eq!(ramp_slide.process(0.4), 0.7166667); // we will ramp down to 0.4 over the remainder of the ramp
    assert_eq!(ramp_slide.process(0.4), 0.5583334);
    assert_eq!(ramp_slide.process(0.4), 0.4);
    assert_eq!(ramp_slide.process(0.4), 0.4);
  }

  #[test]
  fn should_ramp_down_twice_2() {
    let slide_up = 2.0;
    let ramp_down = 4.0;
    let mut ramp_slide = RampSlide::new(1000., slide_up, ramp_down);

    assert_eq!(ramp_slide.process(1.), 1.0);
    assert_eq!(ramp_slide.process(1.), 1.0);
    assert_eq!(ramp_slide.process(0.5), 0.875);
    assert_eq!(ramp_slide.process(0.5), 0.75);
    assert_eq!(ramp_slide.process(0.49), 0.62); // we will ramp down to 0.49 over the remainder of the ramp
    assert_eq!(ramp_slide.process(0.49), 0.49);
    assert_eq!(ramp_slide.process(0.49), 0.49);
  }
}
