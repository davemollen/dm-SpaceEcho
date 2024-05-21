mod utils;
use space_echo::SpaceEcho;
use utils::{generate_signal, get_params};

fn main() {
  let mut space_echo = SpaceEcho::new(44100.);

  let params = get_params();
  let mapped_params = space_echo.map_params(&params);
  space_echo.initialize_params_to_smooth(&mapped_params);

  loop {
    let input = (generate_signal(), generate_signal());
    space_echo.process(input, &mapped_params);
  }
}
