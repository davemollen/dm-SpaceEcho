use space_echo::{FloatExt, SpaceEcho, MIN_DUCK_THRESHOLD};

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn main() {
  let mut space_echo = SpaceEcho::new(44100.);

  let duck_threshold = MIN_DUCK_THRESHOLD.dbtoa();
  let input_level = 1.;
  let feedback = 0.8;
  let wow_and_flutter = 0.777;
  let highpass_freq = 40.;
  let highpass_res = 0.1;
  let lowpass_freq = 6000.;
  let lowpass_res = 0.1;
  let reverb = 0.5;
  let decay = 0.8;
  let stereo = 1.;
  let output_level = 1.;
  let mix = 0.5;
  let hold = false;
  let time_left = 250.;
  let time_right = 250.;
  space_echo.initialize_params(
    input_level,
    feedback,
    wow_and_flutter,
    highpass_freq,
    highpass_res,
    lowpass_freq,
    lowpass_res,
    reverb,
    decay,
    stereo,
    output_level,
    mix,
    time_left,
    time_right,
  );

  loop {
    let input = (generate_signal(), generate_signal());
    space_echo.process(
      input,
      input_level,
      0,
      0,
      time_left,
      time_right,
      false,
      feedback,
      wow_and_flutter,
      highpass_freq,
      highpass_res,
      lowpass_freq,
      lowpass_res,
      reverb,
      decay,
      stereo,
      duck_threshold,
      output_level,
      mix,
      false,
      false,
    );
  }
}
