use crate::slide::Slide;

const ATTACK_TIME: f32 = 3.;
const HOLD_TIME: f32 = 15.;
const RELEASE_TIME: f32 = 40.;
const LIMIT: f32 = 1.;

pub struct Limiter {
  buffer: Vec<(f32, f32)>,
  moving_min_temp: f32,
  moving_min: f32,
  buffer_index: usize,
  hold_index: u32,
  hold_length: u32,
  slide: Slide,
}

impl Limiter {
  pub fn new(sample_rate: f32) -> Self {
    let buffer_length = (ATTACK_TIME * 0.001 * sample_rate + 1.) as usize;
    let hold_length = (HOLD_TIME * 0.001 * sample_rate + 1.) as u32;

    Self {
      buffer: vec![(0., 0.); buffer_length],
      moving_min_temp: 1.,
      moving_min: 1.,
      buffer_index: 0,
      hold_index: 0,
      hold_length,
      slide: Slide::new(sample_rate),
    }
  }

  fn wrap(&self, buffer_index: usize) -> usize {
    let buffer_len = self.buffer.len();
    if buffer_index >= buffer_len {
      buffer_index - buffer_len
    } else {
      buffer_index
    }
  }

  fn get_delay_output(&self) -> (f32, f32) {
    self.buffer[self.buffer_index]
  }

  fn get_gain_reduction(&self, input: (f32, f32)) -> f32 {
    let gain = input.0.abs().max(input.1.abs());
    if gain > LIMIT {
      LIMIT / gain
    } else {
      1.
    }
  }

  fn get_moving_min(&mut self, input: (f32, f32)) -> f32 {
    let gain_reduction = self.get_gain_reduction(input);
    self.moving_min_temp = gain_reduction.min(self.moving_min_temp);

    /*
    Hold on to moving_min for HOLD_TIME when moving_min < 1.
    Unless moving_min_temp < moving_min
     */
    if self.moving_min < 1.
      && self.moving_min < self.moving_min_temp
      && self.hold_index <= self.hold_length
    {
      self.hold_index += 1;
    } else if self.buffer_index == 0 {
      self.moving_min = self.moving_min_temp;
      self.moving_min_temp = 1.;
      self.hold_index = 0;
    }
    self.moving_min
  }

  fn apply_filters(&mut self, moving_min: f32) -> f32 {
    self.slide.run(moving_min, RELEASE_TIME, 0.5)
  }

  fn write_to_buffer(&mut self, input: (f32, f32)) {
    self.buffer[self.buffer_index] = input;
    self.buffer_index = self.wrap(self.buffer_index + 1);
  }

  pub fn run(&mut self, input: (f32, f32)) -> (f32, f32) {
    let delay_output = self.get_delay_output();
    let moving_min = self.get_moving_min(input);
    let limiter_gain = self.apply_filters(moving_min);

    self.write_to_buffer(input);
    (
      (delay_output.0 * limiter_gain).clamp(-LIMIT, LIMIT),
      (delay_output.1 * limiter_gain).clamp(-LIMIT, LIMIT),
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::limiter::Limiter;

  #[test]
  fn get_moving_min() {
    let mut limiter = Limiter::new(1000.);
    assert_eq!(limiter.buffer.len(), 4);

    limiter.buffer_index = 0;
    assert_eq!(limiter.get_moving_min((0.01, 0.01)), 1.);

    limiter.buffer_index = 1;
    assert_eq!(limiter.get_moving_min((0.3, 0.2)), 1.);

    limiter.buffer_index = 2;
    assert_eq!(limiter.get_moving_min((1.4, 0.1)), 1.);

    limiter.buffer_index = 3;
    assert_eq!(limiter.get_moving_min((0.6, 0.1)), 1.);

    limiter.buffer_index = 0;
    assert_eq!(limiter.get_moving_min((0.6, 0.04)), 1.4_f32.recip());

    limiter.buffer_index = 1;
    assert_eq!(limiter.get_moving_min((0.6, 0.04)), 1.4_f32.recip());

    limiter.buffer_index = 2;
    assert_eq!(limiter.get_moving_min((0.6, 2.8)), 1.4_f32.recip());

    limiter.buffer_index = 3;
    assert_eq!(limiter.get_moving_min((1.6, 0.3)), 1.4_f32.recip());

    limiter.buffer_index = 0;
    assert_eq!(limiter.get_moving_min((1.2, 0.3)), 2.8_f32.recip());
  }

  #[test]
  fn hold() {
    let mut limiter = Limiter::new(1000.);
    assert_eq!(limiter.buffer.len(), 4);

    limiter.buffer_index = 0;
    assert_eq!(limiter.get_moving_min((0.01, 0.01)), 1.);

    limiter.buffer_index = 1;
    assert_eq!(limiter.get_moving_min((0.3, 0.2)), 1.);

    limiter.buffer_index = 2;
    assert_eq!(limiter.get_moving_min((1.4, 0.1)), 1.);

    limiter.buffer_index = 3;
    assert_eq!(limiter.get_moving_min((0.6, 0.1)), 1.);

    limiter.buffer_index = 0;
    assert_eq!(limiter.get_moving_min((0.6, 0.04)), 1.4_f32.recip());

    limiter.buffer_index = 1;
    assert_eq!(limiter.get_moving_min((0.6, 0.04)), 1.4_f32.recip());

    limiter.buffer_index = 2;
    assert_eq!(limiter.get_moving_min((0.6, 1.2)), 1.4_f32.recip());

    limiter.buffer_index = 3;
    assert_eq!(limiter.get_moving_min((0.6, 0.3)), 1.4_f32.recip());

    limiter.buffer_index = 0;
    assert_eq!(limiter.get_moving_min((1.2, 0.3)), 1.4_f32.recip());
  }
}
