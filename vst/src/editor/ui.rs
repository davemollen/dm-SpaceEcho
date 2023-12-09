use crate::space_echo_parameters::SpaceEchoParameters;
use std::sync::Arc;
use vizia::{
  prelude::{
    Context, StyleModifiers,
  },
  state::Model,
  views::HStack,
};
use vst::prelude::HostCallback;
mod ui_data;
pub use ui_data::{ParamChangeEvent, UiData};
mod time_controls;
mod reverb_filter_controls;
mod level_controls;

const STYLE: &str = include_str!("./ui/style.css");

pub fn plugin_gui(cx: &mut Context, params: Arc<SpaceEchoParameters>, host: Option<HostCallback>) {
  cx.add_theme(STYLE);

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
