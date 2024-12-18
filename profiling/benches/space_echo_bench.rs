#[path = "../src/utils.rs"]
mod utils;
use criterion::{criterion_group, criterion_main, Criterion};
use space_echo::{SpaceEcho, SmoothParameters};
use utils::{generate_stereo_signal_stream, get_params};

fn space_echo_bench(c: &mut Criterion) {
  let mut space_echo = SpaceEcho::new(44100.);
  let mut smooth_parameters = SmoothParameters::new(44100.);
  let signal_stream = generate_stereo_signal_stream(44100);

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

  c.bench_function("space_echo", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        space_echo.process(
          *signal,
          channel_mode,
          time_mode,
          highpass_res,
          lowpass_res,
          duck_threshold,
          limiter,
          &mut smooth_parameters
        );
      }
    })
  });
}

criterion_group!(benches, space_echo_bench);
criterion_main!(benches);
