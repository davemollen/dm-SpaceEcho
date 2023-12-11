use crate::SpaceEchoParameters;
use nih_plug::prelude::GuiContext;
use std::sync::Arc;
use vizia::{
  prelude::{
    Context, StyleModifiers
  },
  state::Model,
  views::HStack,
};
mod ui_data;
pub use ui_data::{ParamChangeEvent, UiData};
mod time_controls;
mod reverb_filter_controls;
mod level_controls;

const STYLE: &str = include_str!("./ui/style.css");

pub fn plugin_gui(
  cx: &mut Context,
  params: Arc<SpaceEchoParameters>,
  gui_context: Arc<dyn GuiContext>,
) {
  cx.add_theme(STYLE);

  UiData {
    params: params.clone(),
    gui_context: gui_context.clone(),
  }
  .build(cx);

  HStack::new(cx, |cx| {
    time_controls::build(cx, params.clone());
    reverb_filter_controls::build(cx, params.clone());
    level_controls::build(cx, params.clone());
  }).background_color("#161616");
}
