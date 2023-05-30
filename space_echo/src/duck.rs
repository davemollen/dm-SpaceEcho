use crate::slide::Slide;

const ATTACK_TIME: f32 = 1.;
const RELEASE_TIME: f32 = 60.;

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
      let threshold = duck * -0.7061 + 0.7071;
      let summed_side_chain_input = (side_chain_input.0 + side_chain_input.1) * 0.5;
      let side_chain_envelope = summed_side_chain_input.abs();
      let slide_input = if side_chain_envelope > threshold {
        0.14285714
      } else {
        1.
      };
      let duck_gain = self.slide.run(slide_input, RELEASE_TIME, ATTACK_TIME);
      (input.0 * duck_gain, input.1 * duck_gain)
    }
  }
}
