use std::simd::{f32x2, StdFloat};

pub struct Saturation;

impl Saturation {
  pub fn process(input: f32x2, mix: f32) -> f32x2 {
    let mix = (mix * mix).clamp(0., 1.);

    input + (Self::saturate(input) - input) * f32x2::splat(mix)
  }

  fn saturate(x: f32x2) -> f32x2 {
    x / (x * x + f32x2::splat(1.)).sqrt()
  }
}
