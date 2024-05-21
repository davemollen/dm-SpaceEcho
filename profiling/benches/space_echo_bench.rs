#[path = "../src/utils.rs"]
mod utils;
use criterion::{criterion_group, criterion_main, Criterion};
use space_echo::SpaceEcho;
use utils::{generate_stereo_signal_stream, get_params};

fn space_echo_bench(c: &mut Criterion) {
  let mut space_echo = SpaceEcho::new(44100.);
  let signal_stream = generate_stereo_signal_stream(44100);

  let params = get_params();
  let mapped_params = space_echo.map_params(&params);
  space_echo.initialize_params_to_smooth(&mapped_params);

  c.bench_function("space_echo", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        space_echo.process(*signal, &mapped_params);
      }
    })
  });
}

criterion_group!(benches, space_echo_bench);
criterion_main!(benches);
