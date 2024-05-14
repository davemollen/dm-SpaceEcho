use criterion::{criterion_group, criterion_main, Criterion};
use space_echo::SpaceEcho;

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
  let signal_stream = generate_stereo_signal_stream(44100);
  c.bench_function("space_echo", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        space_echo.process(
          *signal, 0., 0, 0, 250., 250., false, 0.8, 0.777, 40., 0.1, 6000., 0.1, 0.5, 0.8, 1., 0.,
          0., 0.5, false, false,
        );
      }
    })
  });
}

criterion_group!(benches, reverb_bench);
criterion_main!(benches);
