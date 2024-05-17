mod average;
use crate::{shared::float_ext::FloatExt, shared::param_filter::ParamFilter};
use average::Average;

pub struct Saturation {
  average: Average,
  enabled: ParamFilter,
}

impl Saturation {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      average: Average::new((1000. / 44100. * sample_rate) as usize),
      enabled: ParamFilter::new(sample_rate, 7.),
    }
  }

  pub fn process(&mut self, input: (f32, f32), threshold: f32) -> (f32, f32, f32) {
    let average = self.average.process((input.0 + input.1) * 0.5);
    let saturation_gain = self
      .enabled
      .process(if average > threshold { 1. } else { 0. });
    let clean_gain = 1. - saturation_gain;
    let saturation_gain_compensation = (1. + threshold - average).clamp(0.4, 1.);

    let (saturation_output_left, saturation_output_right) = (
      self.get_saturation_output(input.0, saturation_gain, clean_gain),
      self.get_saturation_output(input.1, saturation_gain, clean_gain),
    );
    (
      saturation_output_left,
      saturation_output_right,
      saturation_gain_compensation,
    )
  }

  fn get_saturation_output(&self, input: f32, saturation_gain: f32, clean_gain: f32) -> f32 {
    let clean_output = input * clean_gain;

    if saturation_gain > 0. {
      input.fast_tanh1() * saturation_gain + clean_output
    } else {
      clean_output
    }
  }
}
