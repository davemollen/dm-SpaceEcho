mod tap;
use tap::Tap;
mod early_reflection;
use crate::{mix::Mix, phasor::Phasor};
use early_reflection::EarlyReflection;

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
      phasor: Phasor::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: (f32, f32), reverb: f32, decay: f32) -> (f32, f32) {
    if reverb > 0. {
      let early_reflections_out = self.apply_early_reflections(input);
      let delay_network_out = self.read_from_delay_network(early_reflections_out);
      let feedback_matrix_out = self.apply_feedback_matrix(&delay_network_out);
      self.process_and_write_taps(feedback_matrix_out, decay * 0.5);

      Mix::process(input, (delay_network_out[0], delay_network_out[1]), reverb)
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

  fn read_from_delay_network(&mut self, input: (f32, f32)) -> Vec<f32> {
    let lfo_phase = self.phasor.process(3.7);

    self
      .taps
      .iter_mut()
      .zip([input.0, input.1, 0., 0.])
      .map(|(tap, dry_signal)| tap.read(lfo_phase) + dry_signal)
      .collect()
  }

  fn apply_feedback_matrix<'a>(&self, inputs: &'a Vec<f32>) -> impl Iterator<Item = f32> + 'a {
    [
      [1.0, 1.0, 1.0, 1.0],
      [1.0, -1.0, 1.0, -1.0],
      [1.0, 1.0, -1.0, -1.0],
      [1.0, -1.0, -1.0, 1.0],
    ]
    .iter()
    .map(move |feedback_values| -> f32 {
      feedback_values
        .iter()
        .zip(inputs)
        .map(|(feedback, input)| input * feedback)
        .sum()
    })
  }

  fn process_and_write_taps<'a>(
    &'a mut self,
    feedback_matrix_outputs: impl Iterator<Item = f32> + 'a,
    decay: f32,
  ) {
    self
      .taps
      .iter_mut()
      .zip(feedback_matrix_outputs)
      .for_each(|(tap, feedback_matrix_output)| {
        let absorb_output = tap.apply_absorb(feedback_matrix_output);
        tap.write(absorb_output * decay);
      });
  }
}
