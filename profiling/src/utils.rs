use space_echo::MappedParams;

pub fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

pub fn generate_stereo_signal_stream(length: usize) -> Vec<(f32, f32)> {
  (0..length)
    .map(|_| (generate_signal(), generate_signal()))
    .collect()
}

pub fn get_params() -> MappedParams {
  MappedParams {
    input_level: 1.,
    channel_mode: 0,
    time_mode: 0,
    time_left: 250.,
    time_right: 250.,
    feedback: 0.8,
    flutter_gain: 0.2,
    highpass_freq: 40.,
    highpass_res: 0.1,
    lowpass_freq: 6000.,
    lowpass_res: 0.1,
    reverb: 0.5,
    decay: 0.8,
    stereo: 1.,
    duck_threshold: 0.,
    output_level: 1.,
    mix: 0.5,
    limiter: false,
  }
}
