mod smooth;
use crate::{duck::MIN_DUCK_THRESHOLD, FloatExt};
pub use smooth::Smoother;
use smooth::{ExponentialSmooth, LogarithmicSmooth};

const TIME_SMOOTHING_FACTOR: f32 = 0.25;

pub struct Params {
  pub input_level: ExponentialSmooth,
  pub channel_mode: i32,
  pub time_mode: i32,
  time_left: LogarithmicSmooth,
  time_right: LogarithmicSmooth,
  pub feedback: ExponentialSmooth,
  pub flutter_gain: ExponentialSmooth,
  pub highpass_freq: ExponentialSmooth,
  pub highpass_res: f32,
  pub lowpass_freq: ExponentialSmooth,
  pub lowpass_res: f32,
  pub reverb: ExponentialSmooth,
  pub decay: ExponentialSmooth,
  pub stereo: ExponentialSmooth,
  pub duck_threshold: f32,
  pub output_level: ExponentialSmooth,
  pub mix: ExponentialSmooth,
  pub limiter: bool,
  pub filter_fader: ExponentialSmooth,
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      input_level: ExponentialSmooth::new(sample_rate, 7.),
      channel_mode: 0,
      time_mode: 0,
      time_left: LogarithmicSmooth::new(sample_rate, TIME_SMOOTHING_FACTOR),
      time_right: LogarithmicSmooth::new(sample_rate, TIME_SMOOTHING_FACTOR),
      feedback: ExponentialSmooth::new(sample_rate, 7.),
      flutter_gain: ExponentialSmooth::new(sample_rate, 7.),
      highpass_freq: ExponentialSmooth::new(sample_rate, 7.),
      highpass_res: 0.,
      lowpass_freq: ExponentialSmooth::new(sample_rate, 7.),
      lowpass_res: 0.,
      reverb: ExponentialSmooth::new(sample_rate, 7.),
      decay: ExponentialSmooth::new(sample_rate, 7.),
      stereo: ExponentialSmooth::new(sample_rate, 7.),
      duck_threshold: 0.,
      output_level: ExponentialSmooth::new(sample_rate, 7.),
      mix: ExponentialSmooth::new(sample_rate, 7.),
      limiter: false,
      filter_fader: ExponentialSmooth::new(sample_rate, 3.5),
      is_initialized: false,
    }
  }

  pub fn set(
    &mut self,
    input_level: f32,
    channel_mode: i32,
    time_mode: i32,
    time_link: bool,
    time_left: f32,
    time_right: f32,
    feedback: f32,
    wow_and_flutter: f32,
    highpass_freq: f32,
    highpass_res: f32,
    lowpass_freq: f32,
    lowpass_res: f32,
    reverb: f32,
    decay: f32,
    stereo: f32,
    duck: f32,
    output_level: f32,
    mix: f32,
    limiter: bool,
    hold: bool,
  ) {
    self.channel_mode = channel_mode;
    self.time_mode = time_mode;
    self.highpass_res = highpass_res;
    self.lowpass_res = lowpass_res;
    self.duck_threshold = (duck * MIN_DUCK_THRESHOLD).dbtoa();
    self.limiter = limiter;

    let input_level = if hold { 0. } else { input_level.dbtoa() };
    let time_right = if time_link { time_left } else { time_right };
    let feedback = if hold { 1. } else { feedback };
    let flutter_gain = if hold {
      0.
    } else {
      wow_and_flutter * wow_and_flutter * wow_and_flutter
    };
    let decay = decay * 0.5;
    let output_level = output_level.dbtoa();
    let filter_fader = if hold { 0. } else { 1. };

    if self.is_initialized {
      self.input_level.set_target(input_level);
      self.time_left.set_target(time_left);
      self.time_right.set_target(time_right);
      self.feedback.set_target(feedback);
      self.flutter_gain.set_target(flutter_gain);
      self.highpass_freq.set_target(highpass_freq);
      self.lowpass_freq.set_target(lowpass_freq);
      self.reverb.set_target(reverb);
      self.decay.set_target(decay);
      self.stereo.set_target(stereo);
      self.output_level.set_target(output_level);
      self.mix.set_target(mix);
      self.filter_fader.set_target(filter_fader);
    } else {
      self.input_level.reset(input_level);
      self.time_left.reset(time_left);
      self.time_right.reset(time_right);
      self.feedback.reset(feedback);
      self.flutter_gain.reset(flutter_gain);
      self.highpass_freq.reset(highpass_freq);
      self.lowpass_freq.reset(lowpass_freq);
      self.reverb.reset(reverb);
      self.decay.reset(decay);
      self.stereo.reset(stereo);
      self.output_level.reset(output_level);
      self.mix.reset(mix);
      self.filter_fader.reset(filter_fader);
      self.is_initialized = true;
    }
  }

  pub fn get_time(&mut self, time_mode: i32) -> (f32, f32) {
    if time_mode == 0 {
      (self.time_left.next(), self.time_right.next())
    } else {
      (self.time_left.get_target(), self.time_right.get_target())
    }
  }
}
