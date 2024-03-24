use crate::{float_ext::FloatExt, slide::Slide};

const ATTACK_TIME: f32 = 1.;
const RELEASE_TIME: f32 = 30.;

pub struct Duck {
  slide: Slide,
}

impl Duck {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      slide: Slide::new(sample_rate),
    }
  }

  pub fn run(&mut self, input: (f32, f32), side_chain_input: (f32, f32), duck: f32) -> (f32, f32) {
    if duck == 0. {
      input
    } else {
      let threshold = (duck * -40.).dbtoa();
      let summed_side_chain_input = (side_chain_input.0 + side_chain_input.1) * 0.5;
      let slide_input = if summed_side_chain_input.abs() > threshold {
        0.14285714
      } else {
        1.
      };
      let duck_gain = self.slide.run(slide_input, RELEASE_TIME, ATTACK_TIME);
      (input.0 * duck_gain, input.1 * duck_gain)
    }
  }
}
