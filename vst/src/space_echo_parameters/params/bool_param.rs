use super::Params;
use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc,
};

pub struct BoolParam {
  pub name: &'static str,
  pub value: AtomicBool,
  pub default: bool,
  pub index: i32,
  pub value_to_string: Option<Arc<dyn Fn(bool) -> String + Send + Sync>>,
  pub string_to_value: Option<Arc<dyn Fn(&str) -> Option<bool> + Send + Sync>>,
}

impl BoolParam {
  pub fn new(name: &'static str, default: bool, index: i32) -> Self {
    Self {
      name,
      value: AtomicBool::new(default),
      default,
      index,
      value_to_string: None,
      string_to_value: None,
    }
  }
}

impl Params for BoolParam {
  type Plain = bool;

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
    value > 0.5
  }

  fn preview_normalized_value(&self, value: Self::Plain) -> f32 {
    if value {
      1.
    } else {
      0.
    }
  }

  fn set_plain_value(&self, value: Self::Plain) {
    self.value.store(value, Ordering::Relaxed);
  }

  fn set_normalized_value(&self, value: f32) {
    self
      .value
      .store(self.preview_value(value), Ordering::Relaxed);
  }

  fn get_display_value(&self, _: bool) -> String {
    let value = self.value.load(Ordering::Relaxed);
    match (value, &self.value_to_string) {
      (true, None) => String::from("On"),
      (false, None) => String::from("Off"),
      (val, Some(f)) => f(val),
    }
  }

  fn get_default_normalized_value(&self) -> f32 {
    self.preview_normalized_value(self.default)
  }

  fn string_to_normalized_value(&self, string: &str) -> Option<f32> {
    let value = match &self.string_to_value {
      Some(f) => f(string),
      None => Some(string == ("On".to_string())),
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
