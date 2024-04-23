use crate::{moving_min::MovingMin, ramp_slide::RampSlide};

pub struct Limiter {
  buffer: Vec<(f32, f32)>,
  buffer_index: usize,
  slide: RampSlide,
  attack_time: f32,  
  release_time: f32, 
  limit: f32,
  moving_min: MovingMin
}

impl Limiter {
  pub fn new(sample_rate: f32, attack_time: f32, hold_time: f32, release_time: f32, limit: f32) -> Self {
    let buffer_length = (attack_time * 0.001 * sample_rate) as usize;

    Self {
      buffer: vec![(0., 0.); buffer_length],
      buffer_index: 0,
      slide: RampSlide::new(sample_rate),
      attack_time,
      release_time,
      limit,
      moving_min: MovingMin::new(sample_rate, attack_time, hold_time, limit)
    }
  }

  pub fn process(&mut self, input: (f32, f32), is_on: bool) -> (f32, f32) {
    if is_on {
      let moving_min = self.get_moving_min(input);
      let limiter_gain = self.apply_filters(moving_min);
      
      let delay_output = self.read_from_buffer();
      self.write_to_buffer(input);
      (
        (delay_output.0 * limiter_gain).clamp(-self.limit, self.limit),
        (delay_output.1 * limiter_gain).clamp(-self.limit, self.limit),
      )
    } else {
      input
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

  fn read_from_buffer(&self) -> (f32, f32) {
    self.buffer[self.buffer_index]
  }

  fn get_gain_reduction(&self, input: (f32, f32)) -> f32 {
    let gain = input.0.abs().max(input.1.abs());
    if gain > self.limit {
      self.limit / gain
    } else {
      self.limit
    }
  }

  fn get_moving_min(&mut self, input: (f32, f32)) -> f32 {
    let gain_reduction = self.get_gain_reduction(input);
    self.moving_min.process(gain_reduction)
  }

  fn apply_filters(&mut self, moving_min: f32) -> f32 {
    self.slide.process(moving_min, self.release_time, self.attack_time)
  }

  fn write_to_buffer(&mut self, input: (f32, f32)) {
    self.buffer[self.buffer_index] = input;
    self.buffer_index = self.wrap(self.buffer_index + 1);
  }
}