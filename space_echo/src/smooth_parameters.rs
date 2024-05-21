mod log_smooth;
use log_smooth::LogSmooth;

use crate::{shared::param_filter::ParamFilter, MappedParams};

const TIME_SMOOTHING_FACTOR: f32 = 0.25;

pub struct SmoothedParams {
  pub input_level: f32,
  pub feedback: f32,
  pub flutter_gain: f32,
  pub wow_gain: f32,
  pub highpass_freq: f32,
  pub highpass_res: f32,
  pub lowpass_freq: f32,
  pub lowpass_res: f32,
  pub reverb: f32,
  pub decay: f32,
  pub stereo: f32,
  pub output_level: f32,
  pub mix: f32,
  pub time_left: f32,
  pub time_right: f32,
}

pub struct SmoothParameters {
  smooth_input_level: ParamFilter,
  smooth_feedback: ParamFilter,
  smooth_flutter_gain: ParamFilter,
  smooth_highpass_freq: ParamFilter,
  smooth_highpass_res: ParamFilter,
  smooth_lowpass_freq: ParamFilter,
  smooth_lowpass_res: ParamFilter,
  smooth_reverb: ParamFilter,
  smooth_decay: ParamFilter,
  smooth_stereo: ParamFilter,
  smooth_output_level: ParamFilter,
  smooth_mix: ParamFilter,
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
      smooth_highpass_res: ParamFilter::new(sample_rate, 7.),
      smooth_lowpass_freq: ParamFilter::new(sample_rate, 7.),
      smooth_lowpass_res: ParamFilter::new(sample_rate, 7.),
      smooth_reverb: ParamFilter::new(sample_rate, 7.),
      smooth_decay: ParamFilter::new(sample_rate, 7.),
      smooth_stereo: ParamFilter::new(sample_rate, 7.),
      smooth_output_level: ParamFilter::new(sample_rate, 7.),
      smooth_mix: ParamFilter::new(sample_rate, 7.),
      smooth_time_left: LogSmooth::new(sample_rate),
      smooth_time_right: LogSmooth::new(sample_rate),
    }
  }

  pub fn initialize(&mut self, params: &MappedParams) {
    self.smooth_input_level.initialize(params.input_level);
    self.smooth_feedback.initialize(params.feedback);
    self.smooth_flutter_gain.initialize(params.flutter_gain);
    self.smooth_highpass_freq.initialize(params.highpass_freq);
    self.smooth_highpass_res.initialize(params.highpass_res);
    self.smooth_lowpass_freq.initialize(params.lowpass_freq);
    self.smooth_lowpass_res.initialize(params.lowpass_res);
    self.smooth_reverb.initialize(params.reverb);
    self.smooth_decay.initialize(params.decay);
    self.smooth_stereo.initialize(params.stereo);
    self.smooth_output_level.initialize(params.output_level);
    self.smooth_mix.initialize(params.mix);
    self.smooth_time_left.initialize(params.time_left);
    self.smooth_time_right.initialize(params.time_right);
  }

  pub fn get_params(&mut self, params: &MappedParams) -> SmoothedParams {
    let flutter_gain = self.smooth_flutter_gain.process(params.flutter_gain);

    let (time_left, time_right) = if params.time_mode == 0 {
      self.get_time_params(params.time_left, params.time_right)
    } else {
      (params.time_left, params.time_right)
    };

    SmoothedParams {
      input_level: self.smooth_input_level.process(params.input_level),
      feedback: self.smooth_feedback.process(params.feedback),
      flutter_gain,
      wow_gain: flutter_gain * flutter_gain,
      highpass_freq: self.smooth_highpass_freq.process(params.highpass_freq),
      highpass_res: self.smooth_highpass_res.process(params.highpass_res),
      lowpass_freq: self.smooth_lowpass_freq.process(params.lowpass_freq),
      lowpass_res: self.smooth_lowpass_res.process(params.lowpass_res),
      reverb: self.smooth_reverb.process(params.reverb),
      decay: self.smooth_decay.process(params.decay),
      stereo: self.smooth_stereo.process(params.stereo),
      output_level: self.smooth_output_level.process(params.output_level),
      mix: self.smooth_mix.process(params.mix),
      time_left,
      time_right,
    }
  }

  fn get_time_params(&mut self, time_left: f32, time_right: f32) -> (f32, f32) {
    let time_left = self
      .smooth_time_left
      .process(time_left, TIME_SMOOTHING_FACTOR);
    let time_right = self
      .smooth_time_right
      .process(time_right, TIME_SMOOTHING_FACTOR);

    (time_left, time_right)
  }
}
