#[path = "./editor/level_controls.rs"]
mod level_controls;
#[path = "./editor/reverb_filter_controls.rs"]
mod reverb_filter_controls;
#[path = "./editor/time_controls.rs"]
mod time_controls;
mod ui_data;
use crate::space_echo_parameters::SpaceEchoParameters;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::{model::Model, modifiers::StyleModifiers, views::HStack};
use nih_plug_vizia::{create_vizia_editor, vizia_assets, ViziaState, ViziaTheming};
use std::sync::Arc;
pub use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("./editor/style.css");

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
  ViziaState::new(|| (800, 360))
}

pub(crate) fn create(
  params: Arc<SpaceEchoParameters>,
  editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
  create_vizia_editor(
    editor_state,
    ViziaTheming::Custom,
    move |cx, gui_context| {
      vizia_assets::register_roboto(cx);
      vizia_assets::register_roboto_bold(cx);
      cx.set_default_font(&[vizia_assets::ROBOTO]);
      cx.add_stylesheet(STYLE).ok();

      UiData {
        params: params.clone(),
        gui_context: gui_context.clone(),
      }
      .build(cx);

      HStack::new(cx, |cx| {
        time_controls::build(cx, params.clone());
        reverb_filter_controls::build(cx, params.clone());
        level_controls::build(cx, params.clone());
      })
      .background_color("#161616");
    },
  )
}
