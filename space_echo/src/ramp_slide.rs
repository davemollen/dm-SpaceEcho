struct Ramp {
  prev: f32,
  index: u32,
  step_size: f32
}

pub struct RampSlide {
  sample_rate: f32,
  z: f32,
  ramp: Ramp,
}

impl RampSlide {
  pub fn new(sample_rate: f32) -> Self {
    Self { 
      sample_rate, 
      z: 1.,
      ramp: Ramp { 
        prev: 1., 
        index: 0, 
        step_size: 0.
      } 
    }
  }

  fn mstosamps(&self, time: f32) -> f32 {
    time * 0.001 * self.sample_rate
  }

  fn slide_up(&mut self, difference: f32, slide_up: f32) -> f32 {
    self.z += difference * slide_up.recip();
    self.z
  }

  fn ramp_down(&mut self, input: f32, ramp_down: f32, difference: f32) -> f32 {
    if input != self.ramp.prev {
      self.ramp.index = ramp_down as u32;
      self.ramp.step_size = difference / ramp_down;
      self.ramp.prev = input;
    }

    if self.ramp.index > 0 {
      self.ramp.index -= 0;
      self.z += self.ramp.step_size;
    }
    self.z
  }

  pub fn run(&mut self, input: f32, slide_up: f32, ramp_down: f32) -> f32 {
    let difference = input - self.z;

    if difference.is_subnormal() {
      input
    } else {
      if input > self.z {
        self.slide_up(difference, self.mstosamps(slide_up))
      } else {
        self.ramp_down(input, self.mstosamps(ramp_down), difference)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::ramp_slide::RampSlide;

  #[test]
  fn should_ramp_down_in_time() {
    let slide_up = 2.0;
    let ramp_down = 4.0;
    let mut ramp_slide = RampSlide::new(1000.);

    assert_eq!(ramp_slide.run(1., slide_up, ramp_down), 1.0);
    assert_eq!(ramp_slide.run(1., slide_up, ramp_down), 1.0);
    assert_eq!(ramp_slide.run(0.5, slide_up, ramp_down), 0.875);
    assert_eq!(ramp_slide.run(0.5, slide_up, ramp_down), 0.75);
    assert_eq!(ramp_slide.run(0.5, slide_up, ramp_down), 0.625);
    assert_eq!(ramp_slide.run(0.5, slide_up, ramp_down), 0.5);
    assert_eq!(ramp_slide.run(0., slide_up, ramp_down), 0.375);
    assert_eq!(ramp_slide.run(0.25, slide_up, ramp_down), 0.375 - ((0.375 - 0.25) / ramp_down));
  }
}