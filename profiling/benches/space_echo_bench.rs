#[path = "../src/utils.rs"]
mod utils;
use criterion::{criterion_group, criterion_main, Criterion};
use space_echo::SpaceEcho;
use utils::{generate_stereo_signal_stream, get_params};

fn space_echo_bench(c: &mut Criterion) {
  let mut space_echo = SpaceEcho::new(44100.);
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

  c.bench_function("space_echo", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        space_echo.process(
          *signal,
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
    })
  });
}

criterion_group!(benches, space_echo_bench);
criterion_main!(benches);
