pub enum IntRange {
  Linear { min: i32, max: i32 },
  Options { names: Vec<&'static str> },
}

impl IntRange {
  pub fn normalize(&self, value: i32) -> f32 {
    match self {
      IntRange::Linear { min, max } => (value - min) as f32 / (max - min) as f32,
      IntRange::Options { names } => value as f32 / (names.len() - 1) as f32,
    }
  }

  pub fn unnormalize(&self, value: f32) -> i32 {
    match self {
      IntRange::Linear { min, max } => (value * (max - min) as f32).round() as i32 + min,
      IntRange::Options { names } => (value * (names.len() - 1) as f32).round() as i32,
    }
  }
}
