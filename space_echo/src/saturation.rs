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

  fn get_saturation_output(&self, input: f32, saturation_gain: f32, clean_gain: f32) -> f32 {
    let clean_output = input * clean_gain;

    if saturation_gain > 0. {
      input.fast_tanh1() * saturation_gain + clean_output
    } else {
      clean_output
    }
  }

  pub fn run(&mut self, input: (f32, f32), threshold: f32) -> (f32, f32, f32) {
    let average = self.average.run((input.0 + input.1) * 0.5);
    let saturation_gain = self
      .enabled
      .run(if average > threshold { 1. } else { 0. }, 7.);
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
}
