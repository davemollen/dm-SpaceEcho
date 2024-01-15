use crate::space_echo_parameters::SpaceEchoParameters;
use std::sync::Arc;
use vizia::{
  prelude::{
    Context, StyleModifiers,
  },
  views::HStack, 
  model::Model
};
use vst::prelude::HostCallback;
#[path="ui_data.rs"]
mod ui_data;
pub use ui_data::{UiData, ParamChangeEvent};
#[path="time_controls.rs"]
mod time_controls;
#[path="reverb_filter_controls.rs"]
mod reverb_filter_controls;
#[path="level_controls.rs"]
mod level_controls;

const STYLE: &str = include_str!("style.css");

pub fn plugin_gui(cx: &mut Context, params: Arc<SpaceEchoParameters>, host: Option<HostCallback>) {
  let _ = cx.add_stylesheet(STYLE);

  UiData {
    params: params.clone(),
    host,
  }.build(cx);
  
  HStack::new(cx, |cx| {
    time_controls::build(cx, params.clone());
    reverb_filter_controls::build(cx, params.clone());
    level_controls::build(cx, params.clone());
  }).background_color("#161616");
}
