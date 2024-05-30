pub fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

pub fn generate_stereo_signal_stream(length: usize) -> Vec<(f32, f32)> {
  (0..length)
    .map(|_| (generate_signal(), generate_signal()))
    .collect()
}

pub fn get_params() -> (
  f32,
  i32,
  i32,
  f32,
  f32,
  f32,
  f32,
  f32,
  f32,
  f32,
  f32,
  f32,
  f32,
  f32,
  f32,
  f32,
  f32,
  bool,
  f32,
) {
  (
    1., 0, 0, 250., 250., 0.8, 0.2, 40., 0.1, 6000., 0.1, 0.5, 0.8, 1., 0., 1., 0.5, true, 1.,
  )
}
