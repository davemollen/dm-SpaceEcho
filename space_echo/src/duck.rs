use crate::{float_ext::FloatExt, slide::Slide};

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
      let threshold = (duck * -60.).dbtoa();
      let summed_side_chain_input = (side_chain_input.0 + side_chain_input.1) * 0.5;
      let side_chain_envelope = summed_side_chain_input.abs();
      let slide_input = if side_chain_envelope > threshold {
        0.14285714
      } else {
        1.
      };
      let duck_gain = self.slide.run(slide_input, 60., 1.);
      (input.0 * duck_gain, input.1 * duck_gain)
    }
  }
}
