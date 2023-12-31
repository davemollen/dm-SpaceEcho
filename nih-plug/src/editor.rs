#[path="./editor/time_controls.rs"]
mod time_controls;
#[path="./editor/reverb_filter_controls.rs"]
mod reverb_filter_controls;
#[path="./editor/level_controls.rs"]
mod level_controls;
mod ui_data;
pub use ui_data::{UiData, ParamChangeEvent};
use nih_plug::prelude::Editor;
use nih_plug_vizia::{ViziaState, ViziaTheming, create_vizia_editor};
use nih_plug_vizia::vizia::{
  views::HStack, 
  modifiers::StyleModifiers, 
  state::Model
};
use std::sync::Arc;
use crate::space_echo_parameters::SpaceEchoParameters;

const STYLE: &str = include_str!("./editor/style.css");

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (800, 360))
}

pub(crate) fn create(
    params: Arc<SpaceEchoParameters>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, gui_context| { 
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
    })
}
