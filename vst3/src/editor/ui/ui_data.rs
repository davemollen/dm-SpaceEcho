use crate::space_echo_parameters::SpaceEchoParameters;
use nih_plug::prelude::{GuiContext, ParamPtr};
use std::sync::Arc;
use vizia::{
  prelude::{Event, EventContext, Lens},
  state::{Model, Wrapper},
};

pub enum ParamChangeEvent {
  SetParam(ParamPtr, f32),
}

#[derive(Lens)]
pub struct UiData {
  pub params: Arc<SpaceEchoParameters>,
  pub gui_context: Arc<dyn GuiContext>,
}

impl Model for UiData {
  fn event(&mut self, _: &mut EventContext, event: &mut Event) {
    event.map(|app_event, _| match app_event {
      ParamChangeEvent::SetParam(param_ptr, value) => {
        unsafe {
          self
            .gui_context
            .raw_set_parameter_normalized(*param_ptr, *value)
        };
      }
    });
  }
}
