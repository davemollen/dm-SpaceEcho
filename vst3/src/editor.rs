mod ui;
use crate::space_echo_parameters::SpaceEchoParameters;
use nih_plug::prelude::{Editor, GuiContext, ParentWindowHandle};
use std::{
  any::Any,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
};
pub use ui::plugin_gui;
use vizia::{prelude::WindowSize, Application};
mod vizia_editor_handle;
use vizia_editor_handle::ViziaEditorHandle;

const WINDOW_SIZE: WindowSize = WindowSize {
  width: 800,
  height: 360,
};

pub struct SpaceEchoEditor {
  pub params: Arc<SpaceEchoParameters>,
  pub emit_parameters_changed_event: Arc<AtomicBool>,
}

impl Editor for SpaceEchoEditor {
  fn size(&self) -> (u32, u32) {
    (WINDOW_SIZE.width, WINDOW_SIZE.height)
  }

  fn spawn(&self, parent: ParentWindowHandle, context: Arc<dyn GuiContext>) -> Box<dyn Any + Send> {
    let params = self.params.clone();
    let window = Application::new(move |cx| plugin_gui(cx, Arc::clone(&params), context.clone()))
      .title("dm-SpaceEcho")
      .inner_size(WINDOW_SIZE)
      .open_parented(&parent);

    Box::new(ViziaEditorHandle { window })
  }

  fn set_scale_factor(&self, _: f32) -> bool {
    true
  }

  fn param_value_changed(&self, _: &str, _: f32) {
    self
      .emit_parameters_changed_event
      .store(true, Ordering::Relaxed);
  }

  fn param_modulation_changed(&self, _: &str, _: f32) {
    self
      .emit_parameters_changed_event
      .store(true, Ordering::Relaxed);
  }

  fn param_values_changed(&self) {
    self
      .emit_parameters_changed_event
      .store(true, Ordering::Relaxed);
  }
}
