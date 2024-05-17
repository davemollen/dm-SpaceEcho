mod tap;
use tap::Tap;
mod early_reflection;
use crate::shared::{mix::Mix, phasor::Phasor};
use early_reflection::EarlyReflection;

const MATRIX: [[f32; 4]; 4] = [
  [1.0, 1.0, 1.0, 1.0],
  [1.0, -1.0, 1.0, -1.0],
  [1.0, 1.0, -1.0, -1.0],
  [1.0, -1.0, -1.0, 1.0],
];

pub struct Reverb {
  early_reflections: [EarlyReflection; 6],
  taps: [Tap; 4],
  phasor: Phasor,
}

impl Reverb {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      early_reflections: [
        EarlyReflection::new(sample_rate, 5.43216),
        EarlyReflection::new(sample_rate, 8.45346),
        EarlyReflection::new(sample_rate, 13.4367),
        EarlyReflection::new(sample_rate, 21.5463),
        EarlyReflection::new(sample_rate, 34.3876),
        EarlyReflection::new(sample_rate, 55.5437),
      ],
      taps: [
        Tap::new(sample_rate, 60.),
        Tap::new(sample_rate, 71.9345),
        Tap::new(sample_rate, 86.7545),
        Tap::new(sample_rate, 95.945),
      ],
      phasor: Phasor::new(sample_rate, 3.7),
    }
  }

  pub fn process(&mut self, input: (f32, f32), reverb: f32, decay: f32) -> (f32, f32) {
    if reverb > 0. {
      let early_reflections_out = self.apply_early_reflections(input);
      let reverb_out = self.apply_reverb_tail(early_reflections_out, decay * 0.5);

      Mix::process(input, reverb_out, reverb)
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
    let matrix_out = self.apply_matrix(delay_out);
    let absorb_out = self.apply_absorption(matrix_out);
    self.write_to_taps(absorb_out, decay);

    (delay_out[0], delay_out[1])
  }

  fn read_from_taps(&mut self, input: (f32, f32)) -> [f32; 4] {
    let lfo_phase = self.phasor.process();

    [
      self.taps[0].read(lfo_phase) + input.0,
      self.taps[1].read(lfo_phase) + input.1,
      self.taps[2].read(lfo_phase),
      self.taps[3].read(lfo_phase),
    ]
  }

  fn apply_matrix(&self, input: [f32; 4]) -> [f32; 4] {
    [
      self.get_matrix_result(input, MATRIX[0]),
      self.get_matrix_result(input, MATRIX[1]),
      self.get_matrix_result(input, MATRIX[2]),
      self.get_matrix_result(input, MATRIX[3]),
    ]
  }

  fn apply_absorption(&mut self, input: [f32; 4]) -> [f32; 4] {
    [
      self.taps[0].apply_absorb(input[0]),
      self.taps[1].apply_absorb(input[1]),
      self.taps[2].apply_absorb(input[2]),
      self.taps[3].apply_absorb(input[3]),
    ]
  }

  fn write_to_taps(&mut self, input: [f32; 4], decay: f32) {
    self.taps[0].write(input[0] * decay);
    self.taps[1].write(input[1] * decay);
    self.taps[2].write(input[2] * decay);
    self.taps[3].write(input[3] * decay);
  }

  fn get_matrix_result(&self, inputs: [f32; 4], matrix: [f32; 4]) -> f32 {
    inputs
      .iter()
      .zip(matrix)
      .map(|(input, factor)| input * factor)
      .sum()
  }
}
