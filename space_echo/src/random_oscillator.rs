use crate::delta::Delta;
use rand::random;
use std::f32::consts::PI;

pub struct RandomOscillator {
  origin: f32,
  target: f32,
  delta: Delta,
}

impl RandomOscillator {
  pub fn new() -> Self {
    Self {
      origin: 0.,
      target: 0.,
      delta: Delta::new(),
    }
  }

  fn cosine_interp(&self, origin: f32, target: f32, mix: f32) -> f32 {
    let cosine_mix = (1. - (mix * PI).cos()) / 2.;
    origin * (1. - cosine_mix) + target * cosine_mix
  }

  pub fn run(&mut self, phase: f32, probability: f32) -> f32 {
    let trigger = self.delta.run(phase) < 0.;

    if trigger {
      self.origin = self.target;
      self.target = if random::<f32>() <= probability {
        random::<f32>()
      } else {
        0.
      };
    }
    self.cosine_interp(self.origin, self.target, phase)
  }
}
