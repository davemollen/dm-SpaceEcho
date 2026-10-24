mod ramp;
use super::{shared::float_ext::FloatExt, DelayLine, Interpolation};
use ramp::Ramp;

pub struct VariableDelayRead {
  ramp: Ramp,
  previous_time: f32,
  next_time: f32,
}

impl VariableDelayRead {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      ramp: Ramp::new(sample_rate, 5.),
      previous_time: 0.,
      next_time: 0.,
    }
  }

  pub fn reset(&mut self) {
    self.ramp.reset();
  }

  pub fn read(
    &mut self,
    delay_line: &DelayLine,
    time: f32,
    added_time: f32,
    interp: Interpolation,
  ) -> f32 {
    let time_has_changed = time != self.next_time;
    match (time_has_changed, self.ramp.is_finished()) {
      (false, true) => {
        let delay_out = delay_line.read(time + added_time, interp);
        self.next_time = time;
        delay_out
      }
      (true, true) => {
        self.previous_time = self.next_time;
        self.next_time = time;
        self.ramp.start();
        self.crossfade(delay_line, added_time, interp)
      }
      _ => self.crossfade(delay_line, added_time, interp),
    }
  }

  fn crossfade(&mut self, delay_line: &DelayLine, added_time: f32, interp: Interpolation) -> f32 {
    let ramp = self.ramp.process();
    let mix = ramp.cubic_spline_curve();
    let a = delay_line.read(self.previous_time + added_time, interp);
    let b = delay_line.read(self.next_time + added_time, interp);
    a.mix(b, mix)
  }
}
