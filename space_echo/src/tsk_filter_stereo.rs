/* Transposed Sallen Key filter */
mod one_pole_filter_stereo;
use crate::float_ext::FloatExt;
use one_pole_filter_stereo::OnePoleFilterStereo;

#[derive(Clone, Copy)]
pub enum FilterType {
  Lowpass,
  Highpass,
}

pub struct TSKFilterStereo {
  one_pole_filters: [OnePoleFilterStereo; 2],
  z: (f32, f32),
}

impl TSKFilterStereo {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      one_pole_filters: [
        OnePoleFilterStereo::new(sample_rate),
        OnePoleFilterStereo::new(sample_rate),
      ],
      z: (0., 0.),
    }
  }

  pub fn process(
    &mut self,
    input: (f32, f32),
    freq: f32,
    resonance: f32,
    filter_type: FilterType,
  ) -> (f32, f32) {
    let y0 = (input.0 - self.z.0, input.1 - self.z.1);
    let y1 = self.one_pole_filters[0].process(y0, freq, filter_type);
    let y2 = self.one_pole_filters[1].process(y1, freq, filter_type);
    self.z = ((y2.0 - y1.0) * resonance, (y2.1 - y1.1) * resonance);

    y2
  }
}
