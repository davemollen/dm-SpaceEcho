use crate::FloatExt;

pub struct Saturation;

impl Saturation {
  pub fn process(input: (f32, f32), mix: f32) -> (f32, f32) {
    let mix = mix.clamp(0., 1.);
    let saturated = Self::fast_atan2(input);

    (
      input.0 + (saturated.0 - input.0) * mix,
      input.1 + (saturated.1 - input.1) * mix,
    )
  }

  fn fast_atan2(x: (f32, f32)) -> (f32, f32) {
    let n1 = 0.97239411;
    let n2 = -0.19194795;

    (
      ((n1 + n2 * x.0 * x.0) * x.0).clamp(-1., 1.),
      ((n1 + n2 * x.1 * x.1) * x.1).clamp(-1., 1.),
    )
  }
}
