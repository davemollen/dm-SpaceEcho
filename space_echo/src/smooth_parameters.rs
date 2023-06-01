mod log_smooth;
use crate::one_pole_filter::OnePoleFilter;
use log_smooth::LogSmooth;

pub struct SmoothParameters {
  smooth_input_level: OnePoleFilter,
  smooth_feedback: OnePoleFilter,
  smooth_wow_and_flutter: OnePoleFilter,
  smooth_highpass_freq: OnePoleFilter,
  smooth_highpass_res: OnePoleFilter,
  smooth_lowpass_freq: OnePoleFilter,
  smooth_lowpass_res: OnePoleFilter,
  smooth_reverb: OnePoleFilter,
  smooth_decay: OnePoleFilter,
  smooth_stereo: OnePoleFilter,
  smooth_output_level: OnePoleFilter,
  smooth_mix: OnePoleFilter,
  smooth_time_left: LogSmooth,
  smooth_time_right: LogSmooth,
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      smooth_input_level: OnePoleFilter::new(sample_rate),
      smooth_feedback: OnePoleFilter::new(sample_rate),
      smooth_wow_and_flutter: OnePoleFilter::new(sample_rate),
      smooth_highpass_freq: OnePoleFilter::new(sample_rate),
      smooth_highpass_res: OnePoleFilter::new(sample_rate),
      smooth_lowpass_freq: OnePoleFilter::new(sample_rate),
      smooth_lowpass_res: OnePoleFilter::new(sample_rate),
      smooth_reverb: OnePoleFilter::new(sample_rate),
      smooth_decay: OnePoleFilter::new(sample_rate),
      smooth_stereo: OnePoleFilter::new(sample_rate),
      smooth_output_level: OnePoleFilter::new(sample_rate),
      smooth_mix: OnePoleFilter::new(sample_rate),
      smooth_time_left: LogSmooth::new(sample_rate),
      smooth_time_right: LogSmooth::new(sample_rate),
    }
  }

  pub fn get_time_parameters(
    &mut self,
    time_left: f32,
    time_right: f32,
    smoothing_factor: f32,
  ) -> (f32, f32) {
    (
      self.smooth_time_left.run(time_left, smoothing_factor),
      self.smooth_time_right.run(time_right, smoothing_factor),
    )
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
    let input_level = self.smooth_input_level.run(input_level, 7.);
    let feedback = self
      .smooth_feedback
      .run(if hold { 1. } else { feedback }, 7.);
    let wow_and_flutter = self.smooth_wow_and_flutter.run(wow_and_flutter, 7.);
    let highpass_freq = self.smooth_highpass_freq.run(highpass_freq, 7.);
    let highpass_res = self.smooth_highpass_res.run(highpass_res, 7.);
    let lowpass_freq = self.smooth_lowpass_freq.run(lowpass_freq, 7.);
    let lowpass_res = self.smooth_lowpass_res.run(lowpass_res, 7.);
    let reverb = self.smooth_reverb.run(reverb, 7.);
    let decay = self.smooth_decay.run(decay, 7.);
    let stereo = self.smooth_stereo.run(stereo, 7.);
    let output_level = self.smooth_output_level.run(output_level, 7.);
    let mix = self.smooth_mix.run(mix, 7.);

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
}
