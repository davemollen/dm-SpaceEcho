mod log_smooth;
use log_smooth::LogSmooth;

use crate::shared::param_filter::ParamFilter;

const TIME_SMOOTHING_FACTOR: f32 = 0.25;

pub struct SmoothParameters {
  smooth_input_level: ParamFilter,
  smooth_feedback: ParamFilter,
  smooth_flutter_gain: ParamFilter,
  smooth_highpass_freq: ParamFilter,
  smooth_lowpass_freq: ParamFilter,
  smooth_reverb: ParamFilter,
  smooth_decay: ParamFilter,
  smooth_stereo: ParamFilter,
  smooth_output_level: ParamFilter,
  smooth_mix: ParamFilter,
  smooth_filter_gain: ParamFilter,
  smooth_time_left: LogSmooth,
  smooth_time_right: LogSmooth,
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      smooth_input_level: ParamFilter::new(sample_rate, 7.),
      smooth_feedback: ParamFilter::new(sample_rate, 7.),
      smooth_flutter_gain: ParamFilter::new(sample_rate, 7.),
      smooth_highpass_freq: ParamFilter::new(sample_rate, 7.),
      smooth_lowpass_freq: ParamFilter::new(sample_rate, 7.),
      smooth_reverb: ParamFilter::new(sample_rate, 7.),
      smooth_decay: ParamFilter::new(sample_rate, 7.),
      smooth_stereo: ParamFilter::new(sample_rate, 7.),
      smooth_output_level: ParamFilter::new(sample_rate, 7.),
      smooth_mix: ParamFilter::new(sample_rate, 7.),
      smooth_filter_gain: ParamFilter::new(sample_rate, 3.5),
      smooth_time_left: LogSmooth::new(sample_rate, TIME_SMOOTHING_FACTOR),
      smooth_time_right: LogSmooth::new(sample_rate, TIME_SMOOTHING_FACTOR),
    }
  }

  pub fn initialize(
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
    self.smooth_input_level.initialize(input_level);
    self.smooth_feedback.initialize(feedback);
    self.smooth_flutter_gain.initialize(flutter_gain);
    self.smooth_highpass_freq.initialize(highpass_freq);
    self.smooth_lowpass_freq.initialize(lowpass_freq);
    self.smooth_reverb.initialize(reverb);
    self.smooth_decay.initialize(decay);
    self.smooth_stereo.initialize(stereo);
    self.smooth_output_level.initialize(output_level);
    self.smooth_mix.initialize(mix);
    self.smooth_filter_gain.initialize(filter_gain);
    self.smooth_time_left.initialize(time_left);
    self.smooth_time_right.initialize(time_right);
  }

  pub fn get_params(
    &mut self,
    input_level: f32,
    time_mode: i32,
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
  ) -> (
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
  ) {
    let flutter_gain = self.smooth_flutter_gain.process(flutter_gain);

    let (time_left, time_right) = if time_mode == 0 {
      (
        self.smooth_time_left.process(time_left),
        self.smooth_time_right.process(time_right),
      )
    } else {
      (time_left, time_right)
    };

    (
      self.smooth_input_level.process(input_level),
      self.smooth_feedback.process(feedback),
      flutter_gain,
      flutter_gain * flutter_gain,
      self.smooth_highpass_freq.process(highpass_freq),
      self.smooth_lowpass_freq.process(lowpass_freq),
      self.smooth_reverb.process(reverb),
      self.smooth_decay.process(decay),
      self.smooth_stereo.process(stereo),
      self.smooth_output_level.process(output_level),
      self.smooth_mix.process(mix),
      self.smooth_filter_gain.process(filter_gain),
      time_left,
      time_right,
    )
  }
}
