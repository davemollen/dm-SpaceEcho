mod log_smooth;
use log_smooth::LogSmooth;

use crate::shared::param_filter::ParamFilter;

const TIME_SMOOTHING_FACTOR: f32 = 0.25;

pub struct SmoothParameters {
  smooth_input_level: ParamFilter,
  smooth_feedback: ParamFilter,
  smooth_wow_and_flutter: ParamFilter,
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
      smooth_feedback: ParamFilter::new(sample_rate, 3.),
      smooth_wow_and_flutter: ParamFilter::new(sample_rate, 7.),
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

  pub fn initialize(
    &mut self,
    input_level: f32,
    feedback: f32,
    wow_and_flutter: f32,
    highpass_freq: f32,
    highpass_res: f32,
    lowpass_freq: f32,
    lowpass_res: f32,
    reverb: f32,
    decay: f32,
    stereo: f32,
    output_level: f32,
    mix: f32,
    time_left: f32,
    time_right: f32,
  ) {
    self.smooth_input_level.initialize(input_level);
    self.smooth_feedback.initialize(feedback);
    self.smooth_wow_and_flutter.initialize(wow_and_flutter);
    self.smooth_highpass_freq.initialize(highpass_freq);
    self.smooth_highpass_res.initialize(highpass_res);
    self.smooth_lowpass_freq.initialize(lowpass_freq);
    self.smooth_lowpass_res.initialize(lowpass_res);
    self.smooth_reverb.initialize(reverb);
    self.smooth_decay.initialize(decay);
    self.smooth_stereo.initialize(stereo);
    self.smooth_output_level.initialize(output_level);
    self.smooth_mix.initialize(mix);
    self.smooth_time_left.initialize(time_left);
    self.smooth_time_right.initialize(time_right);
  }

  pub fn get_parameters(
    &mut self,
    input_level: f32,
    feedback: f32,
    wow_and_flutter: f32,
    highpass_freq: f32,
    highpass_res: f32,
    lowpass_freq: f32,
    lowpass_res: f32,
    reverb: f32,
    decay: f32,
    stereo: f32,
    output_level: f32,
    mix: f32,
    hold: bool,
  ) -> (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) {
    let input_level = self
      .smooth_input_level
      .process(if hold { 0. } else { input_level });
    let feedback = self
      .smooth_feedback
      .process(if hold { 1. } else { feedback });
    let wow_and_flutter =
      self
        .smooth_wow_and_flutter
        .process(if hold { 0. } else { wow_and_flutter });
    let highpass_freq = self
      .smooth_highpass_freq
      .process(if hold { 20. } else { highpass_freq });
    let highpass_res = self
      .smooth_highpass_res
      .process(if hold { 0. } else { highpass_res });
    let lowpass_freq = self
      .smooth_lowpass_freq
      .process(if hold { 20000. } else { lowpass_freq });
    let lowpass_res = self
      .smooth_lowpass_res
      .process(if hold { 0. } else { lowpass_res });
    let reverb = self.smooth_reverb.process(reverb);
    let decay = self.smooth_decay.process(decay);
    let stereo = self.smooth_stereo.process(stereo);
    let output_level = self.smooth_output_level.process(output_level);
    let mix = self.smooth_mix.process(mix);

    (
      input_level,
      feedback,
      wow_and_flutter,
      highpass_freq,
      highpass_res,
      lowpass_freq,
      lowpass_res,
      reverb,
      decay,
      stereo,
      output_level,
      mix,
    )
  }

  pub fn get_time_parameters(
    &mut self,
    time_left: f32,
    time_right: f32,
    time_link: bool,
  ) -> (f32, f32) {
    let time_left = self
      .smooth_time_left
      .process(time_left, TIME_SMOOTHING_FACTOR);
    let time_right = self.smooth_time_right.process(
      if time_link { time_left } else { time_right },
      TIME_SMOOTHING_FACTOR,
    );

    (time_left, time_right)
  }
}
