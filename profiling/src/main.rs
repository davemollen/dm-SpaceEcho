mod utils;
use space_echo::SpaceEcho;
use utils::{generate_signal, get_params};

fn main() {
  let mut space_echo = SpaceEcho::new(44100.);

  let (
    input_level,
    channel_mode,
    time_mode,
    time_left,
    time_right,
    feedback,
    flutter_gain,
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
    limiter,
    filter_gain,
  ) = get_params();
  space_echo.initialize_params_to_smooth(
    input_level,
    time_left,
    time_right,
    feedback,
    flutter_gain,
    highpass_freq,
    highpass_res,
    lowpass_freq,
    lowpass_res,
    reverb,
    decay,
    stereo,
    output_level,
    mix,
    filter_gain,
  );

  loop {
    let input = (generate_signal(), generate_signal());
    space_echo.process(
      input,
      input_level,
      channel_mode,
      time_mode,
      time_left,
      time_right,
      feedback,
      flutter_gain,
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
      limiter,
      filter_gain,
    );
  }
}
