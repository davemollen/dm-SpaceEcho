/* Transposed Sallen Key filter */

mod one_pole_filter_stereo;
use one_pole_filter_stereo::OnePoleFilterStereo;

#[derive(Clone, Copy)]
pub enum FilterType {
  Lowpass,
  Highpass,
}

pub struct TSKFilterStereo {
  one_pole_filter1: OnePoleFilterStereo,
  one_pole_filter2: OnePoleFilterStereo,
  z: (f32, f32),
}

impl TSKFilterStereo {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      one_pole_filter1: OnePoleFilterStereo::new(sample_rate),
      one_pole_filter2: OnePoleFilterStereo::new(sample_rate),
      z: (0., 0.),
    }
  }

  fn get_filter_output(
    one_pole_filter: &mut OnePoleFilterStereo,
    input: (f32, f32),
    freq: f32,
    filter_type: FilterType,
  ) -> (f32, f32) {
    match filter_type {
      FilterType::Lowpass => one_pole_filter.run(input, freq),
      FilterType::Highpass => {
        let filter_output = one_pole_filter.run(input, freq);
        (input.0 - filter_output.0, input.1 - filter_output.1)
      }
    }
  }

  pub fn run(
    &mut self,
    input: (f32, f32),
    freq: f32,
    resonance: f32,
    filter_type: FilterType,
  ) -> (f32, f32) {
    let y0 = (input.0 - self.z.0, input.1 - self.z.1);
    let y1 = Self::get_filter_output(&mut self.one_pole_filter1, y0, freq, filter_type);
    let y2 = Self::get_filter_output(&mut self.one_pole_filter2, y1, freq, filter_type);
    self.z = ((y2.0 - y1.0) * resonance, (y2.1 - y1.1) * resonance);

    y2
  }
}
