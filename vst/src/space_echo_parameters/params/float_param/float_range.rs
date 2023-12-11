pub enum FloatRange {
  Linear {
    min: f32,
    max: f32,
  },
  Skewed {
    min: f32,
    max: f32,
    factor: f32,
  },
  AsymmetricalSkewed {
    min: f32,
    max: f32,
    center: f32,
    below_center_factor: f32,
    above_center_factor: f32,
  },
}

impl FloatRange {
  pub fn normalize(&self, value: f32) -> f32 {
    match self {
      FloatRange::Linear { min, max } => (value.clamp(*min, *max) - min) / (max - min),
      FloatRange::Skewed { min, max, factor } => {
        ((value.clamp(*min, *max) - min) / (max - min)).powf(*factor)
      }
      FloatRange::AsymmetricalSkewed {
        min,
        max,
        center,
        below_center_factor,
        above_center_factor,
      } => {
        let unscaled_proportion = (value.clamp(*min, *max) - min) / (max - min);
        let center_proportion = (center - min) / (max - min);
        if unscaled_proportion > center_proportion {
          let scaled_proportion =
            (unscaled_proportion - center_proportion) * (1.0 - center_proportion).recip();
          (scaled_proportion.powf(*above_center_factor) * 0.5) + 0.5
        } else {
          let inverted_scaled_proportion =
            (center_proportion - unscaled_proportion) * (center_proportion).recip();

          (1.0 - inverted_scaled_proportion.powf(*below_center_factor)) * 0.5
        }
      }
    }
  }

  pub fn unnormalize(&self, value: f32) -> f32 {
    match self {
      FloatRange::Linear { min, max } => (value * (max - min)) + min,
      FloatRange::Skewed { min, max, factor } => (value.powf(factor.recip()) * (max - min)) + min,
      FloatRange::AsymmetricalSkewed {
        min,
        max,
        center,
        below_center_factor,
        above_center_factor,
      } => {
        let center_proportion = (center - min) / (max - min);
        let skewed_proportion = if value > 0.5 {
          let scaled_proportion = (value - 0.5) * 2.0;
          (scaled_proportion.powf(above_center_factor.recip()) * (1.0 - center_proportion))
            + center_proportion
        } else {
          let inverted_scaled_proportion = (0.5 - value) * 2.0;
          (1.0 - inverted_scaled_proportion.powf(below_center_factor.recip())) * center_proportion
        };

        (skewed_proportion * (max - min)) + min
      }
    }
  }
}
