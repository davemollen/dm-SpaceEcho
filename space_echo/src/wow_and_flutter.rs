use crate::{
  float_ext::FloatExt, one_pole_filter::OnePoleFilter, phasor::Phasor,
  random_oscillator::RandomOscillator,
};

const MAX_FLUTTER_TIME_IN_SECS: f32 = 2.5;
const MAX_WOW_TIME_IN_SECS: f32 = 8.;
pub const MAX_WOW_AND_FLUTTER_TIME_IN_SECS: f32 = MAX_FLUTTER_TIME_IN_SECS + MAX_WOW_TIME_IN_SECS;

pub struct WowAndFlutter {
  wow_phasor: Phasor,
  wow_oscillator: RandomOscillator,
  wow_modulator_phasor: Phasor,
  wow_modulator: RandomOscillator,
  flutter_phasor: Phasor,
  flutter_oscillator: RandomOscillator,
  smooth_param: OnePoleFilter,
}

impl WowAndFlutter {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      wow_phasor: Phasor::new(sample_rate),
      wow_oscillator: RandomOscillator::new(),
      wow_modulator_phasor: Phasor::new(sample_rate),
      wow_modulator: RandomOscillator::new(),
      flutter_phasor: Phasor::new(sample_rate),
      flutter_oscillator: RandomOscillator::new(),
      smooth_param: OnePoleFilter::new(sample_rate),
    }
  }

  pub fn get_wow_oscillator(&mut self) -> f32 {
    let wow_modulator_phase = self.wow_modulator_phasor.run(0.2);
    let wow_oscillator_freq = self.wow_modulator.run(wow_modulator_phase, 1.) * 2. + 0.5;

    let wow_oscillator_phase = self.wow_phasor.run(wow_oscillator_freq);
    self.wow_oscillator.run(wow_oscillator_phase, 0.2) * MAX_WOW_TIME_IN_SECS
  }

  pub fn get_flutter_oscillator(&mut self) -> f32 {
    let flutter_oscillator_phase = self.flutter_phasor.run(27.37891);
    self.flutter_oscillator.run(flutter_oscillator_phase, 0.9) * MAX_FLUTTER_TIME_IN_SECS
  }

  pub fn run(&mut self, wow_and_flutter: f32) -> f32 {
    let wow_oscillator = self.get_wow_oscillator();
    let flutter_oscillator = self.get_flutter_oscillator();

    let scaled_param = wow_and_flutter.fast_pow(3.);
    let smoothed_param = self.smooth_param.run(scaled_param, 12.);

    (wow_oscillator + flutter_oscillator) * smoothed_param
  }
}
