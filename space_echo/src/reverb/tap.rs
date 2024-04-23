use crate::{
  delay_line::{DelayLine, Interpolation},
  one_pole_filter::OnePoleFilter,
  random_oscillator::RandomOscillator,
};

pub struct Tap {
  time_in_ms: f32,
  delay_line: DelayLine,
  one_pole_filter: OnePoleFilter,
  random_lfo: RandomOscillator,
}

impl Tap {
  pub fn new(sample_rate: f32, time_in_ms: f32) -> Self {
    Self {
      time_in_ms,
      delay_line: DelayLine::new(
        (sample_rate * (time_in_ms + 1.) / 1000.) as usize + 1,
        sample_rate,
      ),
      one_pole_filter: OnePoleFilter::new(sample_rate),
      random_lfo: RandomOscillator::new(),
    }
  }

  pub fn write(&mut self, input: f32) {
    self.delay_line.write(input);
  }

  pub fn read(&mut self, lfo_phase: f32) -> f32 {
    let time_offset = self.random_lfo.process(lfo_phase, 1.);

    self
      .delay_line
      .read(self.time_in_ms + time_offset, Interpolation::Linear)
  }

  pub fn apply_absorb(&mut self, input: f32) -> f32 {
    self.one_pole_filter.process(input, 6000.)
  }
}
