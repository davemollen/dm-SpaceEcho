mod space_echo_parameters;
use space_echo_parameters::SpaceEchoParameters;
use std::sync::Arc;
use vizia::prelude::*;
mod editor;
use editor::plugin_gui;

fn main() {
  let params = Arc::new(SpaceEchoParameters::default());

  Application::new(move |cx| plugin_gui(cx, Arc::clone(&params), None))
    .title("dm-SpaceEcho")
    .inner_size((720, 360))
    .run();
}
