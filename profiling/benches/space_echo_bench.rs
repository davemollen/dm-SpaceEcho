#[path = "../src/utils.rs"]
mod utils;
use criterion::{criterion_group, criterion_main, Criterion};
use space_echo::{Params, SpaceEcho};
use utils::generate_stereo_signal_stream;

fn space_echo_bench(c: &mut Criterion) {
  let mut space_echo = SpaceEcho::new(44100.);
  let mut params = Params::new(44100.);
  params.set(
    1., 0, 0, false, 250., 250., 0.8, 0.2, 40., 0.1, 6000., 0.1, 0.5, 0.8, 1., 0., 1., 0.5, true,
    false,
  );
  let signal_stream = generate_stereo_signal_stream(44100);

  c.bench_function("space_echo", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        space_echo.process(*signal, &mut params);
      }
    })
  });
}

criterion_group!(benches, space_echo_bench);
criterion_main!(benches);
