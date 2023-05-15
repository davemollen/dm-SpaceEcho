mod boxcar_filter;
mod slew_release;
use boxcar_filter::BoxcarFilter;
use slew_release::SlewRelease;

const ATTACK_TIME: f32 = 3.;
const HOLD_TIME: f32 = 15.;
const RELEASE_TIME: f32 = 40.;
const LIMIT: f32 = 1.;

pub struct Limiter {
  buffer: Vec<(f32, f32)>,
  moving_min_temp: f32,
  moving_min: f32,
  buffer_index: usize,
  box_carfilter: BoxcarFilter,
  slew_release: SlewRelease,
}

impl Limiter {
  pub fn new(sample_rate: f32) -> Self {
    let buffer_length = (ATTACK_TIME * 0.001 * sample_rate + 1.) as usize;

    Self {
      buffer: vec![(0., 0.); buffer_length],
      moving_min_temp: 1.,
      moving_min: 1.,
      buffer_index: 0,
      box_carfilter: BoxcarFilter::new(buffer_length),
      slew_release: SlewRelease::new(sample_rate),
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
    let output = self.buffer[self.wrap(self.buffer_index + 1)];
    (output.0, output.1)
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
    /*
    Hold on to moving_in for x samples when moving_in < 1.
    Unless moving_min_temp < moving_in
     */
    let gain_reduction = self.get_gain_reduction(input);
    self.moving_min_temp = gain_reduction.min(self.moving_min_temp);

    if self.buffer_index == 0 {
      self.moving_min = self.moving_min_temp;
      self.moving_min_temp = 1.;
    }
    self.moving_min
  }

  fn apply_filters(&mut self, moving_min: f32) -> f32 {
    let slewed_moving_min = self.slew_release.run(moving_min, RELEASE_TIME);
    self.box_carfilter.run(slewed_moving_min)
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
    (delay_output.0 * limiter_gain, delay_output.1 * limiter_gain)
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
}
