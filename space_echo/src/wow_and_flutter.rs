use crate::{
  float_ext::FloatExt, one_pole_filter::OnePoleFilter, phasor::Phasor,
  random_oscillator::RandomOscillator,
};

const MAX_FLUTTER_TIME_IN_SECS: f32 = 2.;
const MAX_WOW_TIME_IN_SECS: f32 = 15.;
pub const MAX_WOW_AND_FLUTTER_TIME_IN_SECS: f32 = MAX_FLUTTER_TIME_IN_SECS + MAX_WOW_TIME_IN_SECS;

pub struct WowAndFlutter {
  wow_phasor: Phasor,
  wow_oscillator: RandomOscillator,
  flutter_phasor: Phasor,
  flutter_oscillator: RandomOscillator,
  smooth_param: OnePoleFilter,
}

impl WowAndFlutter {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      wow_phasor: Phasor::new(sample_rate),
      wow_oscillator: RandomOscillator::new(),
      flutter_phasor: Phasor::new(sample_rate),
      flutter_oscillator: RandomOscillator::new(),
      smooth_param: OnePoleFilter::new(sample_rate),
    }
  }

  pub fn get_wow_oscillator(&mut self) -> f32 {
    let wow_oscillator_phase = self.wow_phasor.run(2.1);
    self.wow_oscillator.run(wow_oscillator_phase, 0.4) * MAX_WOW_TIME_IN_SECS
  }

  pub fn get_flutter_oscillator(&mut self) -> f32 {
    let flutter_oscillator_phase = self.flutter_phasor.run(24.37891);
    self.flutter_oscillator.run(flutter_oscillator_phase, 0.95) * MAX_FLUTTER_TIME_IN_SECS
  }

  pub fn run(&mut self, wow_and_flutter: f32) -> f32 {
    let wow_oscillator = self.get_wow_oscillator();
    let flutter_oscillator = self.get_flutter_oscillator();

    let smoothed_param = self.smooth_param.run(wow_and_flutter, 12.);
    let flutter_gain = smoothed_param.fast_pow(3.);
    let wow_gain = smoothed_param.fast_pow(6.);

    wow_oscillator * wow_gain + flutter_oscillator * flutter_gain
  }
}
