mod early_reflection;
mod one_pole_filter;

use {
  crate::shared::{
    delay_line::{DelayLine, Interpolation},
    mix::Mix,
    phasor::Phasor,
    random_oscillator::RandomOscillator,
  },
  early_reflection::EarlyReflection,
  one_pole_filter::OnePoleFilter,
  std::simd::f32x4,
};

const MATRIX: [[f32; 4]; 4] = [
  [1.0, 1.0, 1.0, 1.0],
  [1.0, -1.0, 1.0, -1.0],
  [1.0, 1.0, -1.0, -1.0],
  [1.0, -1.0, -1.0, 1.0],
];

pub struct Reverb {
  early_reflections: [EarlyReflection; 6],
  time: [f32; 4],
  delay_line: [DelayLine; 4],
  one_pole_filter: OnePoleFilter,
  random_lfo: [RandomOscillator; 4],
  phasor: Phasor,
  mix: Mix,
}

impl Reverb {
  pub fn new(sample_rate: f32) -> Self {
    let delay_times = [60., 71.9345, 86.7545, 95.945];

    Self {
      early_reflections: [
        EarlyReflection::new(sample_rate, 5.43216),
        EarlyReflection::new(sample_rate, 8.45346),
        EarlyReflection::new(sample_rate, 13.4367),
        EarlyReflection::new(sample_rate, 21.5463),
        EarlyReflection::new(sample_rate, 34.3876),
        EarlyReflection::new(sample_rate, 55.5437),
      ],
      time: delay_times,
      delay_line: delay_times
        .map(|time| DelayLine::new((sample_rate * time / 1000.) as usize + 1, sample_rate)),
      one_pole_filter: OnePoleFilter::new(sample_rate, 6000.),
      random_lfo: [RandomOscillator::new(); 4],
      phasor: Phasor::new(sample_rate, 3.7),
      mix: Mix::new(),
    }
  }

  pub fn process(&mut self, input: (f32, f32), reverb: f32, decay: f32) -> (f32, f32) {
    if reverb > 0. {
      let early_reflections_out = self.apply_early_reflections(input);
      let reverb_out = self.apply_reverb_tail(early_reflections_out, decay);

      self.mix.process(input, reverb_out, reverb)
    } else {
      input
    }
  }

  fn apply_early_reflections(&mut self, input: (f32, f32)) -> (f32, f32) {
    let early_reflections_out = self
      .early_reflections
      .iter_mut()
      .fold(input, |sum, early_reflection| early_reflection.process(sum));

    (
      early_reflections_out.0 * 0.125,
      early_reflections_out.1 * 0.125,
    )
  }

  fn apply_reverb_tail(&mut self, input: (f32, f32), decay: f32) -> (f32, f32) {
    let delay_out = self.read_from_taps(input);
    let matrix_out = Self::apply_matrix(delay_out);
    self.apply_absorption_and_write_to_taps(matrix_out, decay);

    (delay_out[0], delay_out[1])
  }

  fn read_from_taps(&mut self, input: (f32, f32)) -> [f32; 4] {
    let phase = self.phasor.process();

    [
      self.delay_line[0].read(
        self.time[0] + self.random_lfo[0].process(phase, 1.),
        Interpolation::Linear,
      ) + input.0,
      self.delay_line[1].read(
        self.time[1] + self.random_lfo[1].process(phase, 1.),
        Interpolation::Linear,
      ) + input.1,
      self.delay_line[2].read(
        self.time[2] + self.random_lfo[2].process(phase, 1.),
        Interpolation::Linear,
      ),
      self.delay_line[3].read(
        self.time[3] + self.random_lfo[3].process(phase, 1.),
        Interpolation::Linear,
      ),
    ]
  }

  fn apply_absorption_and_write_to_taps(&mut self, input: f32x4, decay: f32) {
    let absorb_out = self.one_pole_filter.process(input);

    absorb_out
      .to_array()
      .into_iter()
      .enumerate()
      .for_each(|(i, x)| self.delay_line[i].write(x * decay));
  }

  fn apply_matrix(input: [f32; 4]) -> f32x4 {
    [
      Self::get_matrix_result(input, MATRIX[0]),
      Self::get_matrix_result(input, MATRIX[1]),
      Self::get_matrix_result(input, MATRIX[2]),
      Self::get_matrix_result(input, MATRIX[3]),
    ]
    .into()
  }

  fn get_matrix_result(inputs: [f32; 4], matrix: [f32; 4]) -> f32 {
    inputs
      .into_iter()
      .zip(matrix)
      .map(|(input, factor)| input * factor)
      .sum()
  }
}
