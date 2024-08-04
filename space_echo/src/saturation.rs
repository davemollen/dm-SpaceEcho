pub struct Saturation;

impl Saturation {
  pub fn process(input: (f32, f32), mix: f32) -> (f32, f32) {
    let mix = mix.clamp(0., 1.);

    (
      input.0 + (Self::saturate(input.0) - input.0) * mix,
      input.1 + (Self::saturate(input.1) - input.1) * mix,
    )
  }

  fn saturate(x: f32) -> f32 {
    if x < -2.65155 {
      1.
    } else if x > 2.65155 {
      -1.
    } else {
      (0.97239411 - 0.19194795 * x * x) * x
    }
  }
}
