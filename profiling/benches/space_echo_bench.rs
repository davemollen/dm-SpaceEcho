use criterion::{criterion_group, criterion_main, Criterion};
use space_echo::{FloatExt, SpaceEcho, MIN_DUCK_THRESHOLD};

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn generate_stereo_signal_stream(length: usize) -> Vec<(f32, f32)> {
  (0..length)
    .map(|_| (generate_signal(), generate_signal()))
    .collect()
}

fn reverb_bench(c: &mut Criterion) {
  let mut space_echo = SpaceEcho::new(44100.);
  let duck_threshold = MIN_DUCK_THRESHOLD.dbtoa();
  let signal_stream = generate_stereo_signal_stream(44100);

  let time_left = 250.;
  let time_right = 250.;
  space_echo.initialize_params(time_left, time_right);

  c.bench_function("space_echo", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        space_echo.process(
          *signal,
          1.,
          0,
          0,
          time_left,
          time_right,
          false,
          0.8,
          0.777,
          40.,
          0.1,
          6000.,
          0.1,
          0.5,
          0.8,
          1.,
          duck_threshold,
          1.,
          0.5,
          false,
          false,
        );
      }
    })
  });
}

criterion_group!(benches, reverb_bench);
criterion_main!(benches);
