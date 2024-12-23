mod utils;
use space_echo::{SmoothParameters, SpaceEcho};
use utils::generate_signal;

fn main() {
  let mut space_echo = SpaceEcho::new(44100.);
  let mut smooth_parameters = SmoothParameters::new(44100.);
  smooth_parameters.set_targets(
    1., 0, 0, 250., 250., 0.8, 0.2, 40., 0.1, 6000., 0.1, 0.5, 0.8, 1., 0., 1., 0.5, true, 1.,
  );

  loop {
    let input = (generate_signal(), generate_signal());
    space_echo.process(
      input,
      &mut smooth_parameters
    );
  }
}
