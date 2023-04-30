use std::sync::Arc;
mod bool_param;
pub use bool_param::BoolParam;
mod float_param;
pub use float_param::{FloatParam, FloatRange};
mod int_param;
pub use int_param::{IntParam, IntRange};

pub trait Params {
  type Plain: PartialEq;

  fn get_index(&self) -> i32;
  fn get_value(&self) -> Self::Plain;
  fn get_normalized_value(&self) -> f32;
  fn preview_value(&self, value: f32) -> Self::Plain;
  fn preview_normalized_value(&self, value: Self::Plain) -> f32;
  fn set_plain_value(&self, value: Self::Plain);
  fn set_normalized_value(&self, value: f32);
  fn get_display_value(&self, include_unit: bool) -> String;
  fn get_default_normalized_value(&self) -> f32;
  fn string_to_normalized_value(&self, string: &str) -> Option<f32>;
  fn with_value_to_string(self, callback: Arc<dyn Fn(Self::Plain) -> String + Send + Sync>)
    -> Self;
  fn with_string_to_value(
    self,
    callback: Arc<dyn Fn(&str) -> Option<Self::Plain> + Send + Sync>,
  ) -> Self;
}
