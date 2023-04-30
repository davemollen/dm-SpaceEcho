use std::sync::Arc;
use vst::util::AtomicFloat;
mod float_range;
use super::Params;
pub use float_range::FloatRange;

pub struct FloatParam {
  pub name: &'static str,
  pub value: AtomicFloat,
  pub default: f32,
  pub index: i32,
  pub range: FloatRange,
  pub unit: &'static str,
  pub value_to_string: Option<Arc<dyn Fn(f32) -> String + Send + Sync>>,
  pub string_to_value: Option<Arc<dyn Fn(&str) -> Option<f32> + Send + Sync>>,
}

impl FloatParam {
  pub fn new(name: &'static str, default: f32, index: i32, range: FloatRange) -> Self {
    Self {
      name,
      value: AtomicFloat::new(default),
      default,
      index,
      range,
      unit: "",
      value_to_string: None,
      string_to_value: None,
    }
  }

  pub fn with_unit(mut self, unit: &'static str) -> Self {
    self.unit = unit;
    self
  }
}

impl Params for FloatParam {
  type Plain = f32;

  fn get_index(&self) -> i32 {
    self.index
  }

  fn get_value(&self) -> Self::Plain {
    self.value.get()
  }

  fn get_normalized_value(&self) -> f32 {
    self.range.normalize(self.get_value())
  }

  fn preview_value(&self, value: f32) -> Self::Plain {
    self.range.unnormalize(value)
  }

  fn preview_normalized_value(&self, value: Self::Plain) -> f32 {
    self.range.normalize(value)
  }

  fn set_plain_value(&self, value: Self::Plain) {
    let plain_value = self.range.unnormalize(value);
    self.value.set(plain_value);
  }

  fn set_normalized_value(&self, value: f32) {
    let normalized_value = self.preview_normalized_value(value);
    self.value.set(normalized_value);
  }

  fn get_display_value(&self, include_unit: bool) -> String {
    let value = self.get_value();
    match (&self.value_to_string, include_unit) {
      (Some(f), true) => format!("{}{}", f(value), self.unit),
      (Some(f), false) => f(value),
      (None, true) => format!("{}{}", value, self.unit),
      (None, false) => value.to_string(),
    }
  }

  fn get_default_normalized_value(&self) -> f32 {
    self.range.normalize(self.default)
  }

  fn string_to_normalized_value(&self, string: &str) -> Option<f32> {
    let value = match &self.string_to_value {
      Some(f) => f(string),
      None => string.trim().trim_end_matches(self.unit).parse().ok(),
    }?;

    Some(self.preview_normalized_value(value))
  }

  fn with_value_to_string(
    mut self,
    callback: Arc<dyn Fn(Self::Plain) -> String + Send + Sync>,
  ) -> Self {
    self.value_to_string = Some(callback);
    self
  }

  fn with_string_to_value(
    mut self,
    callback: Arc<dyn Fn(&str) -> Option<Self::Plain> + Send + Sync>,
  ) -> Self {
    self.string_to_value = Some(callback);
    self
  }
}
