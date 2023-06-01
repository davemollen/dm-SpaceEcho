use crate::{average::Average, float_ext::FloatExt, one_pole_filter::OnePoleFilter};

pub struct Saturation {
  average: Average,
  enabled: OnePoleFilter,
}

impl Saturation {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      average: Average::new((1000. / 44100. * sample_rate) as usize),
      enabled: OnePoleFilter::new(sample_rate),
    }
  }

  pub fn run(&mut self, input: (f32, f32), threshold: f32) -> (f32, f32, f32) {
    let average = self.average.run((input.0 + input.1) * 0.5);
    let factor = self
      .enabled
      .run(if average > threshold { 1. } else { 0. }, 7.);
    let inverted_factor = 1. - factor;
    let saturation_gain_compensation = (1. + threshold - average).clamp(0.5, 1.);

    (
      input.0.fast_tanh1() * factor + input.0 * inverted_factor,
      input.1.fast_tanh1() * factor + input.1 * inverted_factor,
      saturation_gain_compensation,
    )
  }
}
