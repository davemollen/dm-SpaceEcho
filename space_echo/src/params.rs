mod smooth;
use smooth::{ExponentialSmooth, LogarithmicSmooth};
pub use smooth::Smoother;
use crate::{FloatExt, duck::MIN_DUCK_THRESHOLD};

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
  pub filter_gain: ExponentialSmooth,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      input_level: ExponentialSmooth::new(7., sample_rate),
      channel_mode: 0,
      time_mode: 0,
      time_left: LogarithmicSmooth::new(TIME_SMOOTHING_FACTOR, sample_rate),
      time_right: LogarithmicSmooth::new(TIME_SMOOTHING_FACTOR, sample_rate),
      feedback: ExponentialSmooth::new(7., sample_rate),
      flutter_gain: ExponentialSmooth::new(7., sample_rate),
      highpass_freq: ExponentialSmooth::new(7., sample_rate),
      highpass_res: 0.,
      lowpass_freq: ExponentialSmooth::new(7., sample_rate),
      lowpass_res: 0.,
      reverb: ExponentialSmooth::new(7., sample_rate),
      decay: ExponentialSmooth::new(7., sample_rate),
      stereo: ExponentialSmooth::new(7., sample_rate),
      duck_threshold: 0.,
      output_level: ExponentialSmooth::new(7., sample_rate),
      mix: ExponentialSmooth::new(7., sample_rate),
      limiter: false,
      filter_gain: ExponentialSmooth::new(3.5, sample_rate),
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
    self.input_level.set_target(if hold { 0. } else { input_level.dbtoa() });
    self.channel_mode = channel_mode;
    self.time_mode = time_mode;
    self.time_left.set_target(time_left);
    self.time_right.set_target(if time_link {
      time_left
    } else {
      time_right
    });
    self.feedback.set_target(if hold { 1. } else { feedback });
    self.flutter_gain.set_target(if hold {
      0.
    } else {
      wow_and_flutter * wow_and_flutter * wow_and_flutter
    });
    self.highpass_freq.set_target(highpass_freq);
    self.highpass_res = highpass_res;
    self.lowpass_freq.set_target(lowpass_freq);
    self.lowpass_res = lowpass_res;
    self.reverb.set_target(reverb);
    self.decay.set_target(decay * 0.5);
    self.stereo.set_target(stereo);
    self.duck_threshold = (duck * MIN_DUCK_THRESHOLD).dbtoa();
    self.output_level.set_target(output_level.dbtoa());
    self.mix.set_target(mix);
    self.limiter = limiter;
    self.filter_gain.set_target(if hold { 0. } else { 1. });
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
