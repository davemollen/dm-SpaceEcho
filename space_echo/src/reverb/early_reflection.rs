use crate::shared::delay_line::{DelayLine, Interpolation};

pub struct EarlyReflection {
  time_in_ms: f32,
  delay_line: DelayLine,
}

impl EarlyReflection {
  pub fn new(sample_rate: f32, time_in_ms: f32) -> Self {
    Self {
      time_in_ms,
      delay_line: DelayLine::new((time_in_ms * sample_rate) as usize, sample_rate),
    }
  }

  pub fn process(&mut self, input: (f32, f32)) -> (f32, f32) {
    let added_channels = input.0 + input.1;
    let subtracted_channels = input.0 - input.1;
    let delay_out = self.delay_line.read(self.time_in_ms, Interpolation::Step);
    self.delay_line.write(subtracted_channels);

    (added_channels, delay_out)
  }
}
