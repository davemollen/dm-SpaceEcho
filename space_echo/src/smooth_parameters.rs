mod smooth;
use smooth::{ExponentialSmooth, LogarithmicSmooth};
pub use smooth::Smoother;

const TIME_SMOOTHING_FACTOR: f32 = 0.25;

pub struct SmoothParameters {
  pub input_level: ExponentialSmooth,
  pub feedback: ExponentialSmooth,
  pub flutter_gain: ExponentialSmooth,
  pub highpass_freq: ExponentialSmooth,
  pub lowpass_freq: ExponentialSmooth,
  pub reverb: ExponentialSmooth,
  pub decay: ExponentialSmooth,
  pub stereo: ExponentialSmooth,
  pub output_level: ExponentialSmooth,
  pub mix: ExponentialSmooth,
  pub filter_gain: ExponentialSmooth,
  time_left: LogarithmicSmooth,
  time_right: LogarithmicSmooth,
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      input_level: ExponentialSmooth::new(7., sample_rate),
      feedback: ExponentialSmooth::new(7., sample_rate),
      flutter_gain: ExponentialSmooth::new(7., sample_rate),
      highpass_freq: ExponentialSmooth::new(7., sample_rate),
      lowpass_freq: ExponentialSmooth::new(7., sample_rate),
      reverb: ExponentialSmooth::new(7., sample_rate),
      decay: ExponentialSmooth::new(7., sample_rate),
      stereo: ExponentialSmooth::new(7., sample_rate),
      output_level: ExponentialSmooth::new(7., sample_rate),
      mix: ExponentialSmooth::new(7., sample_rate),
      filter_gain: ExponentialSmooth::new(3.5, sample_rate),
      time_left: LogarithmicSmooth::new(TIME_SMOOTHING_FACTOR, sample_rate),
      time_right: LogarithmicSmooth::new(TIME_SMOOTHING_FACTOR, sample_rate),
    }
  }

  pub fn set_targets(
    &mut self,
    input_level: f32,
    time_left: f32,
    time_right: f32,
    feedback: f32,
    flutter_gain: f32,
    highpass_freq: f32,
    lowpass_freq: f32,
    reverb: f32,
    decay: f32,
    stereo: f32,
    output_level: f32,
    mix: f32,
    filter_gain: f32,
  ) {
    self.input_level.set_target(input_level);
    self.feedback.set_target(feedback);
    self.flutter_gain.set_target(flutter_gain);
    self.highpass_freq.set_target(highpass_freq);
    self.lowpass_freq.set_target(lowpass_freq);
    self.reverb.set_target(reverb);
    self.decay.set_target(decay);
    self.stereo.set_target(stereo);
    self.output_level.set_target(output_level);
    self.mix.set_target(mix);
    self.filter_gain.set_target(filter_gain);
    self.time_left.set_target(time_left);
    self.time_right.set_target(time_right);
  }

  pub fn get_time(&mut self, time_mode: i32) -> (f32, f32) {
    if time_mode == 0 {
      (
        self.time_left.next(),
        self.time_right.next(),
      )
    } else {
      (self.time_left.get_target(), self.time_right.get_target())
    }
  }
}
