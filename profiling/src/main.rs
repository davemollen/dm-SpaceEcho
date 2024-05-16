use space_echo::SpaceEcho;

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn main() {
  let mut space_echo = SpaceEcho::new(44100.);

  loop {
    let input = (generate_signal(), generate_signal());
    space_echo.process(
      input, 0., 0, 0, 250., 250., false, 0.8, 0.777, 40., 0.1, 6000., 0.1, 0.5, 0.8, 1., 0., 0.,
      0.5, false, false,
    );
  }
}
