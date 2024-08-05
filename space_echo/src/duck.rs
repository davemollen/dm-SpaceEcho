mod slide;
use crate::shared::float_ext::FloatExt;
use slide::Slide;

const ATTACK_TIME: f32 = 1.5;
const RELEASE_TIME: f32 = 120.;
pub const MIN_DUCK_THRESHOLD: f32 = -60.;
const MAX_DUCK_THRESHOLD: f32 = 0.;

pub struct Duck {
  max_duck_threshold: f32,
  slide: Slide,
}

impl Duck {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      max_duck_threshold: MAX_DUCK_THRESHOLD.dbtoa(),
      slide: Slide::new(sample_rate, RELEASE_TIME, ATTACK_TIME),
    }
  }

  pub fn process(
    &mut self,
    input: (f32, f32),
    side_chain_input: (f32, f32),
    duck_threshold: f32,
  ) -> (f32, f32) {
    if duck_threshold == self.max_duck_threshold {
      input
    } else {
      let peak = Self::detect_peak(side_chain_input);
      let slide_input = if peak > duck_threshold {
        duck_threshold / peak
      } else {
        1.
      };
      let duck_gain = self.slide.process(slide_input);
      (input.0 * duck_gain, input.1 * duck_gain)
    }
  }

  fn detect_peak(input: (f32, f32)) -> f32 {
    input.0.abs().max(input.1.abs())
  }
}
