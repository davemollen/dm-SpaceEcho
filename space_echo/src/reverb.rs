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
      let reverb_out = self.apply_reverb_tail(early_reflections_out, decay);

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
    let matrix_out = Self::apply_matrix(delay_out);
    self.apply_absorption_and_write_to_taps(matrix_out, decay);

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

  fn apply_matrix(input: [f32; 4]) -> impl IntoIterator<Item = f32> {
    MATRIX
      .into_iter()
      .map(move |matrix_element| Self::get_matrix_result(input, matrix_element))
  }

  fn apply_absorption_and_write_to_taps(
    &mut self,
    input: impl IntoIterator<Item = f32>,
    decay: f32,
  ) {
    input
      .into_iter()
      .zip(self.taps.iter_mut())
      .for_each(|(x, tap)| {
        let absorb_out = tap.apply_absorb(x);
        tap.write(absorb_out * decay)
      })
  }

  fn get_matrix_result(inputs: [f32; 4], matrix: [f32; 4]) -> f32 {
    inputs
      .into_iter()
      .zip(matrix)
      .map(|(input, factor)| input * factor)
      .sum()
  }
}
