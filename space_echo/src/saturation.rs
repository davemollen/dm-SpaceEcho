pub struct Saturation;

impl Saturation {
  pub fn process(input: (f32, f32), mix: f32) -> (f32, f32) {
    let mix = (mix * mix).clamp(0., 1.);

    (
      input.0 + (Self::saturate(input.0) - input.0) * mix,
      input.1 + (Self::saturate(input.1) - input.1) * mix,
    )
  }

  fn saturate(x: f32) -> f32 {
    x / (x * x + 1.).sqrt()
  }
}
