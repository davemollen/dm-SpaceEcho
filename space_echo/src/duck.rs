mod slide;
use crate::{average::Average, float_ext::FloatExt};
use slide::Slide;

pub struct Duck {
  average: Average,
  slide: Slide,
}

impl Duck {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      average: Average::new(1000),
      slide: Slide::new(sample_rate),
    }
  }

  pub fn run(&mut self, input: (f32, f32), side_chain_input: (f32, f32), duck: f32) -> (f32, f32) {
    if duck == 0. {
      input
    } else {
      let threshold = (duck * -64. - 6.).dbtoa();
      let gain_reduction = (duck * -58. - 12.).dbtoa();
      let summed_side_chain_input = (side_chain_input.0 + side_chain_input.1) * 0.5;
      let average = self.average.run(summed_side_chain_input);
      let slide_input = if average > threshold {
        gain_reduction
      } else {
        1.
      };
      let duck_gain = self.slide.run(slide_input, 6., 120.);
      (input.0 * duck_gain, input.1 * duck_gain)
    }
  }
}
