mod space_echo_parameters;
use space_echo_parameters::SpaceEchoParameters;
use std::sync::Arc;
use vizia::prelude::*;
mod editor;
use editor::{plugin_gui, WINDOW_SIZE};

fn main() {
  let params = Arc::new(SpaceEchoParameters::default());

  Application::new(move |cx| plugin_gui(cx, Arc::clone(&params), None))
    .title("dm-SpaceEcho")
    .inner_size((WINDOW_SIZE.width, WINDOW_SIZE.height))
    .run();
}
