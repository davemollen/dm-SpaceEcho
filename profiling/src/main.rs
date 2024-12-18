mod utils;
use space_echo::{SmoothParameters, SpaceEcho};
use utils::{generate_signal, get_params};

fn main() {
  let mut space_echo = SpaceEcho::new(44100.);
  let mut smooth_parameters = SmoothParameters::new(44100.);

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
  smooth_parameters.set_targets(input_level, time_left, time_right, feedback, flutter_gain, highpass_freq, lowpass_freq, reverb, decay, stereo, output_level, mix, filter_gain);

  loop {
    let input = (generate_signal(), generate_signal());
    space_echo.process(
      input,
      channel_mode,
      time_mode,
      highpass_res,
      lowpass_res,
      duck_threshold,
      limiter,
      &mut smooth_parameters
    );
  }
}
