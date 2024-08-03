#![feature(portable_simd)]
mod average;
mod dc_block_stereo;
mod duck;
mod limiter;
mod reverb;
mod saturation;
mod smooth_parameters;
mod tsk_filter_stereo;
mod variable_delay_read;
mod wow_and_flutter;
mod shared {
  pub mod delay_line;
  pub mod delta;
  pub mod float_ext;
  pub mod mix;
  pub mod param_filter;
  pub mod phasor;
  pub mod random_oscillator;
  pub mod slide;
}

use {
  average::Average,
  dc_block_stereo::DcBlockStereo,
  duck::Duck,
  limiter::Limiter,
  saturation::Saturation,
  shared::{
    delay_line::{DelayLine, Interpolation},
    mix::Mix,
  },
  smooth_parameters::SmoothParameters,
  std::simd::f32x2,
  tsk_filter_stereo::{FilterType, TSKFilterStereo},
  variable_delay_read::VariableDelayRead,
  wow_and_flutter::{WowAndFlutter, MAX_WOW_AND_FLUTTER_TIME_IN_SECS},
};
pub use {duck::MIN_DUCK_THRESHOLD, reverb::Reverb, shared::float_ext::FloatExt};

pub struct SpaceEcho {
  delay_line_left: DelayLine,
  delay_line_right: DelayLine,
  variable_delay_read_left: VariableDelayRead,
  variable_delay_read_right: VariableDelayRead,
  wow_and_flutter: WowAndFlutter,
  average: Average,
  highpass_filter: TSKFilterStereo,
  lowpass_filter: TSKFilterStereo,
  reverb: Reverb,
  duck: Duck,
  dc_block: DcBlockStereo,
  limiter: Limiter,
  smooth_parameters: SmoothParameters,
}

impl SpaceEcho {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line_left: DelayLine::new(
        ((2.5 + MAX_WOW_AND_FLUTTER_TIME_IN_SECS) * sample_rate) as usize,
        sample_rate,
      ),
      delay_line_right: DelayLine::new(
        ((2.5 + MAX_WOW_AND_FLUTTER_TIME_IN_SECS) * sample_rate) as usize,
        sample_rate,
      ),
      variable_delay_read_left: VariableDelayRead::new(sample_rate),
      variable_delay_read_right: VariableDelayRead::new(sample_rate),
      wow_and_flutter: WowAndFlutter::new(sample_rate),
      average: Average::new(21_f32.mstosamps(sample_rate) as usize),
      highpass_filter: TSKFilterStereo::new(sample_rate),
      lowpass_filter: TSKFilterStereo::new(sample_rate),
      reverb: Reverb::new(sample_rate),
      duck: Duck::new(sample_rate),
      dc_block: DcBlockStereo::new(sample_rate),
      limiter: Limiter::new(sample_rate, 2., 10., 40., 0.966051),
      smooth_parameters: SmoothParameters::new(sample_rate),
    }
  }

  pub fn initialize_params_to_smooth(
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
    self.smooth_parameters.initialize(
      input_level,
      time_left,
      time_right,
      feedback,
      flutter_gain,
      highpass_freq,
      lowpass_freq,
      reverb,
      decay,
      stereo,
      output_level,
      mix,
      filter_gain,
    );
  }

  pub fn process(
    &mut self,
    input: (f32, f32),
    input_level: f32,
    channel_mode: i32,
    time_mode: i32,
    time_left: f32,
    time_right: f32,
    feedback: f32,
    flutter_gain: f32,
    highpass_freq: f32,
    highpass_res: f32,
    lowpass_freq: f32,
    lowpass_res: f32,
    reverb: f32,
    decay: f32,
    stereo: f32,
    duck_threshold: f32,
    output_level: f32,
    mix: f32,
    limiter: bool,
    filter_gain: f32,
  ) -> (f32, f32) {
    let (
      input_level,
      feedback,
      wow_gain,
      flutter_gain,
      highpass_freq,
      lowpass_freq,
      reverb,
      decay,
      stereo,
      output_level,
      mix,
      filter_gain,
      time_left,
      time_right,
    ) = self.smooth_parameters.get_params(
      input_level,
      time_mode,
      time_left,
      time_right,
      feedback,
      flutter_gain,
      highpass_freq,
      lowpass_freq,
      reverb,
      decay,
      stereo,
      output_level,
      mix,
      filter_gain,
    );

    let delay_input = self.get_delay_input(input, channel_mode, input_level);
    let delay_output =
      self.read_from_delay_lines(time_left, time_right, time_mode, wow_gain, flutter_gain);

    let average = self
      .average
      .process(Self::take_loudest_channel(delay_output));
    let gain_compensation = if average > 0.4 { 0.4 / average } else { 1. };

    let filter_output = self.apply_filter(
      delay_output,
      highpass_freq,
      highpass_res,
      lowpass_freq,
      lowpass_res,
      filter_gain,
    );
    let feedback_matrix_output = self.apply_channel_mode(filter_output, channel_mode);
    let db_block_output = self.dc_block.process(feedback_matrix_output);
    self.write_to_delay_lines(delay_input, db_block_output, feedback, average);

    let stereo_output = self.apply_stereo_amount(filter_output, stereo);
    let reverb_output = self.reverb.process(
      self.apply_gain(stereo_output, gain_compensation),
      reverb,
      decay,
    );
    let ducking_output = self.duck.process(reverb_output, input, duck_threshold);
    let space_echo_output = self.apply_gain(ducking_output, output_level);
    let mix_output = Mix::process(input, space_echo_output, mix);
    self.limiter.process(mix_output, limiter)
  }

  fn apply_gain(&self, input: (f32, f32), gain: f32) -> (f32, f32) {
    (input.0 * gain, input.1 * gain)
  }

  fn get_delay_input(&self, input: (f32, f32), channel_mode: i32, gain: f32) -> (f32, f32) {
    let input = self.apply_gain(input, gain);

    match channel_mode {
      1 => ((input.0 + input.1) * 0.5, 0.),
      _ => input,
    }
  }

  fn read_from_delay_lines(
    &mut self,
    time_left: f32,
    time_right: f32,
    time_mode: i32,
    wow_gain: f32,
    flutter_gain: f32,
  ) -> (f32, f32) {
    let wow_and_flutter_time = if wow_gain > 0. {
      self.wow_and_flutter.process(wow_gain, flutter_gain)
    } else {
      0.
    };

    if time_mode == 0 {
      let delay_out_left = self
        .delay_line_left
        .read(time_left + wow_and_flutter_time, Interpolation::Linear);
      let delay_out_right = self
        .delay_line_right
        .read(time_right + wow_and_flutter_time, Interpolation::Linear);

      (delay_out_left, delay_out_right)
    } else {
      let delay_out_left = self.variable_delay_read_left.read(
        &mut self.delay_line_left,
        time_left,
        wow_and_flutter_time,
        Interpolation::Linear,
      );
      let delay_out_right = self.variable_delay_read_right.read(
        &mut self.delay_line_right,
        time_right,
        wow_and_flutter_time,
        Interpolation::Linear,
      );

      (delay_out_left, delay_out_right)
    }
  }

  fn apply_filter(
    &mut self,
    input: (f32, f32),
    highpass_freq: f32,
    highpass_res: f32,
    lowpass_freq: f32,
    lowpass_res: f32,
    filter_gain: f32,
  ) -> (f32, f32) {
    match filter_gain {
      0. => input,
      1. => {
        let filter_out = self.get_filter_output(
          input,
          highpass_freq,
          highpass_res,
          lowpass_freq,
          lowpass_res,
        );
        (filter_out[0], filter_out[1])
      }
      _ => {
        let filter_out = self.get_filter_output(
          input,
          highpass_freq,
          highpass_res,
          lowpass_freq,
          lowpass_res,
        );
        let input_gain = 1. - filter_gain;
        (
          filter_out[0] * filter_gain + input.0 * input_gain,
          filter_out[1] * filter_gain + input.1 * input_gain,
        )
      }
    }
  }

  fn get_filter_output(
    &mut self,
    input: (f32, f32),
    highpass_freq: f32,
    highpass_res: f32,
    lowpass_freq: f32,
    lowpass_res: f32,
  ) -> [f32; 2] {
    let highpass_filter_out = self.highpass_filter.process(
      f32x2::from_array([input.0, input.1]),
      highpass_freq,
      highpass_res,
      FilterType::Highpass,
    );
    self
      .lowpass_filter
      .process(
        highpass_filter_out,
        lowpass_freq,
        lowpass_res,
        FilterType::Lowpass,
      )
      .to_array()
  }

  fn apply_channel_mode(&mut self, input: (f32, f32), channel_mode: i32) -> (f32, f32) {
    match channel_mode {
      1 => (input.1, input.0),
      _ => input,
    }
  }

  fn write_to_delay_lines(
    &mut self,
    dry_input: (f32, f32),
    feedback_input: (f32, f32),
    feedback: f32,
    saturation_mix: f32,
  ) {
    let feedback_output = (
      dry_input.0 + feedback_input.0 * feedback,
      dry_input.1 + feedback_input.1 * feedback,
    );
    let saturation_output = Saturation::process(feedback_output, saturation_mix);

    self.delay_line_left.write(saturation_output.0);
    self.delay_line_right.write(saturation_output.1);
  }

  fn apply_stereo_amount(&self, input: (f32, f32), stereo: f32) -> (f32, f32) {
    let factor = (1. - stereo) * 0.5;
    let inverted_factor = 1. - factor;

    (
      input.0 * inverted_factor + input.1 * factor,
      input.0 * factor + input.1 * inverted_factor,
    )
  }

  fn take_loudest_channel(input: (f32, f32)) -> f32 {
    if input.0.abs() > input.1.abs() {
      input.0
    } else {
      input.1
    }
  }
}
