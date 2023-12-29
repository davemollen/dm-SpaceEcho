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

  fn get_saturation_output(&self, input: f32, saturation_gain: f32, feedback: f32) -> f32 {
    let clean_gain = 1. - saturation_gain;
    let clean_output = match (clean_gain > 0., feedback >= 1.) {
      (true, true) => (input * clean_gain).clamp(-1., 1.),
      (true, false) => input * clean_gain,
      _ => 0.
    };

    let saturation_output = match saturation_gain > 0. {
      true => input.fast_tanh1() * saturation_gain,
      false => 0.
    };

    clean_output + saturation_output
  }

  pub fn run(&mut self, input: (f32, f32), feedback: f32, threshold: f32) -> (f32, f32, f32) {
    let average = self.average.run((input.0 + input.1) * 0.5);
    let factor = self
      .enabled
      .run(if average > threshold { 1. } else { 0. }, 7.);
    let saturation_gain_compensation = (1. + threshold - average).clamp(0.4, 1.);

    let (saturation_output_left, saturation_output_right) = (
      self.get_saturation_output(input.0, factor, feedback),
      self.get_saturation_output(input.1, factor, feedback),
    );
    (
      saturation_output_left,
      saturation_output_right,
      saturation_gain_compensation,
    )
  }
}
