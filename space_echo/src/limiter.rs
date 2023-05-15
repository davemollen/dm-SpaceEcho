use crate::slide::Slide;

const ATTACK_TIME: f32 = 3.;
const RELEASE_TIME: f32 = 60.;
const THRESHOLD: f32 = 0.891251;

pub struct Limiter {
  buffer: Vec<(f32, f32)>,
  max_gain: f32,
  index: usize,
  slide: Slide,
}

impl Limiter {
  pub fn new(sample_rate: f32) -> Self {
    let buffer_length = (ATTACK_TIME * 0.001 * sample_rate + 1.) as usize;

    Self {
      buffer: vec![(0., 0.); buffer_length],
      max_gain: 0.,
      index: 0,
      slide: Slide::new(sample_rate),
    }
  }

  fn wrap(&self, index: usize) -> usize {
    let buffer_len = self.buffer.len();
    if index >= buffer_len {
      index - buffer_len
    } else {
      index
    }
  }

  fn get_max_gain(&mut self) -> f32 {
    if self.index == 0 {
      self.max_gain = self.buffer.iter().fold(0., |max, item| {
        let item_max = item.0.max(item.1);
        item_max.max(max)
      });
      self.max_gain
    } else {
      self.max_gain
    }
  }

  fn get_limiter_gain(&mut self, max_gain: f32) -> f32 {
    let limiter_gain = if max_gain > THRESHOLD {
      THRESHOLD / max_gain
    } else {
      1.
    };
    // let slewed_limiter_gain = self.slew_release.run(limiter_gain, RELEASE_TIME);
    // self.box_carfilter.run(slewed_limiter_gain)
    self.slide.run(limiter_gain, RELEASE_TIME, ATTACK_TIME)
  }

  fn write_to_buffer(&mut self, input: (f32, f32)) {
    self.buffer[self.index] = input;
    self.index = self.wrap(self.index + 1);
  }

  fn read_from_buffer(&self) -> (f32, f32) {
    self.buffer[self.wrap(self.index + 1)]
  }

  pub fn run(&mut self, input: (f32, f32)) -> (f32, f32) {
    let max_gain = self.get_max_gain();
    let limiter_gain = self.get_limiter_gain(max_gain);
    let delayed_input = self.read_from_buffer();

    self.write_to_buffer(input);
    (
      delayed_input.0 * limiter_gain,
      delayed_input.1 * limiter_gain,
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::limiter::Limiter;

  #[test]
  fn get_max_gain() {
    let mut limiter = Limiter::new(1250.);
    assert_eq!(limiter.buffer.len(), 4);

    assert_eq!(limiter.index, 0);
    assert_eq!(limiter.max_gain, 0.);
    limiter.run((0., 0.));
    limiter.run((0.3, 0.2));
    limiter.run((0.8, 0.1));
    limiter.run((0.3, 0.05));
    limiter.run((0.3, 0.05));
    assert_eq!(limiter.index, 1);
    assert_eq!(limiter.max_gain, 0.8);

    limiter.run((0.8, 1.4));
    assert_eq!(limiter.max_gain, 0.8);
    limiter.run((0.2, 0.1));
    limiter.run((0.2, 0.1));
    limiter.run((0.2, 0.1));
    assert_eq!(limiter.max_gain, 1.4);
  }
}
