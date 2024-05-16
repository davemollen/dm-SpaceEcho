use crate::{float_ext::FloatExt, slide::Slide, MIN_DUCK_THRESHOLD};

const ATTACK_TIME: f32 = 1.5;
const RELEASE_TIME: f32 = 60.;

pub struct Duck {
  min_duck_threshold: f32,
  slide: Slide,
}

impl Duck {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      min_duck_threshold: MIN_DUCK_THRESHOLD.dbtoa(),
      slide: Slide::new(sample_rate),
    }
  }

  pub fn process(
    &mut self,
    input: (f32, f32),
    side_chain_input: (f32, f32),
    duck_threshold: f32,
  ) -> (f32, f32) {
    if duck_threshold == self.min_duck_threshold {
      input
    } else {
      let summed_side_chain_input = (side_chain_input.0 + side_chain_input.1) * 0.5;
      let slide_input = if summed_side_chain_input.abs() > duck_threshold {
        0.14285714
      } else {
        1.
      };
      let duck_gain = self.slide.process(slide_input, RELEASE_TIME, ATTACK_TIME);
      (input.0 * duck_gain, input.1 * duck_gain)
    }
  }
}
