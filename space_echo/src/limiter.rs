mod moving_min;
mod ramp_slide;
use {moving_min::MovingMin, ramp_slide::RampSlide};

pub struct Limiter {
  buffer: Vec<(f32, f32)>,
  buffer_index: usize,
  slide: RampSlide,
  limit: f32,
  moving_min: MovingMin,
}

impl Limiter {
  pub fn new(
    sample_rate: f32,
    attack_time: f32,
    hold_time: f32,
    release_time: f32,
    limit: f32,
  ) -> Self {
    let buffer_length = (attack_time * 0.001 * sample_rate) as usize;

    Self {
      buffer: vec![(0., 0.); buffer_length],
      buffer_index: 0,
      slide: RampSlide::new(sample_rate, release_time, attack_time),
      limit,
      moving_min: MovingMin::new(sample_rate, attack_time, hold_time, limit),
    }
  }

  pub fn process(&mut self, input: (f32, f32), is_on: bool) -> (f32, f32) {
    if is_on {
      let limiter_gain = self.get_limiter_gain(input);
      self.write_to_buffer(input);
      let delay_output = self.read_from_buffer();

      (delay_output.0 * limiter_gain, delay_output.1 * limiter_gain)
    } else {
      input
    }
  }

  fn get_limiter_gain(&mut self, input: (f32, f32)) -> f32 {
    let gain_reduction = self.get_gain_reduction(input);
    let moving_min = self.moving_min.process(gain_reduction);
    let limiter_gain = self.slide.process(moving_min);
    limiter_gain
  }

  fn wrap(&self, buffer_index: usize) -> usize {
    let buffer_len = self.buffer.len();
    if buffer_index >= buffer_len {
      buffer_index - buffer_len
    } else {
      buffer_index
    }
  }

  fn read_from_buffer(&self) -> (f32, f32) {
    self.buffer[self.buffer_index]
  }

  fn get_gain_reduction(&self, input: (f32, f32)) -> f32 {
    let gain = input.0.abs().max(input.1.abs());
    if gain > self.limit {
      self.limit * gain.recip()
    } else {
      1.
    }
  }

  fn write_to_buffer(&mut self, input: (f32, f32)) {
    self.buffer[self.buffer_index] = input;
    self.buffer_index = self.wrap(self.buffer_index + 1);
  }
}
