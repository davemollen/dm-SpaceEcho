use std::sync::{
  atomic::{AtomicI32, Ordering},
  Arc,
};
mod int_range;
use super::Params;
pub use int_range::IntRange;

pub struct IntParam {
  pub name: &'static str,
  pub value: AtomicI32,
  pub default: i32,
  pub index: i32,
  pub range: IntRange,
  pub unit: &'static str,
  pub value_to_string: Option<Arc<dyn Fn(i32) -> String + Send + Sync>>,
  pub string_to_value: Option<Arc<dyn Fn(&str) -> Option<i32> + Send + Sync>>,
  pub labels: Option<&'static [&'static str]>,
}

impl IntParam {
  pub fn new(name: &'static str, default: i32, index: i32, range: IntRange) -> Self {
    Self {
      name,
      value: AtomicI32::new(default),
      default,
      index,
      range,
      unit: "",
      value_to_string: None,
      string_to_value: None,
      labels: None,
    }
  }

  pub fn get_options(&self) -> Vec<String> {
    match &self.range {
      IntRange::Options { names } => names.iter().map(|name| name.to_string()).collect(),
      _ => vec!["".to_string()],
    }
  }
}

impl Params for IntParam {
  type Plain = i32;

  fn get_index(&self) -> i32 {
    self.index
  }

  fn get_value(&self) -> Self::Plain {
    self.value.load(Ordering::Relaxed)
  }

  fn get_normalized_value(&self) -> f32 {
    self.preview_normalized_value(self.get_value())
  }

  fn preview_value(&self, value: f32) -> Self::Plain {
    self.range.unnormalize(value)
  }

  fn preview_normalized_value(&self, value: Self::Plain) -> f32 {
    self.range.normalize(value)
  }

  fn set_plain_value(&self, value: Self::Plain) {
    self.value.store(value, Ordering::Relaxed);
  }

  fn set_normalized_value(&self, value: f32) {
    self
      .value
      .store(self.preview_value(value), Ordering::Relaxed);
  }

  fn get_display_value(&self, include_unit: bool) -> String {
    let value = self.get_value();

    match (self.labels, &self.value_to_string, include_unit) {
      (Some(labels), _, _) => labels[value as usize].to_string(),
      (None, Some(f), true) => format!("{}{}", f(value), self.unit),
      (None, Some(f), false) => f(value),
      (None, None, true) => format!("{}{}", value, self.unit),
      (None, None, false) => value.to_string(),
    }
  }

  fn get_default_normalized_value(&self) -> f32 {
    self.preview_normalized_value(self.default)
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
