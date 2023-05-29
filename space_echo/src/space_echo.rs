use crate::{
  average::Average,
  dc_block_stereo::DcBlockStereo,
  delay_line::{DelayLine, Interpolation},
  duck::Duck,
  float_ext::FloatExt,
  limiter::Limiter,
  log_smooth::LogSmooth,
  mix::Mix,
  one_pole_filter::OnePoleFilter,
  reverb::Reverb,
  tsk_filter_stereo::{FilterType, TSKFilterStereo},
  variable_delay_read::VariableDelayRead,
  wow_and_flutter::{WowAndFlutter, MAX_WOW_AND_FLUTTER_TIME_IN_SECS},
};

const SATURATION_THRESHOLD: f32 = 0.15;

pub struct SpaceEcho {
  delay_line_left: DelayLine,
  delay_line_right: DelayLine,
  variable_delay_read_left: VariableDelayRead,
  variable_delay_read_right: VariableDelayRead,
  wow_and_flutter: WowAndFlutter,
  highpass_filter: TSKFilterStereo,
  lowpass_filter: TSKFilterStereo,
  average: Average,
  saturation_enabled: OnePoleFilter,
  reverb: Reverb,
  duck: Duck,
  dc_block: DcBlockStereo,
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
  limiter: Limiter,
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
      highpass_filter: TSKFilterStereo::new(sample_rate),
      lowpass_filter: TSKFilterStereo::new(sample_rate),
      average: Average::new((1000. / 44100. * sample_rate) as usize),
      saturation_enabled: OnePoleFilter::new(sample_rate),
      reverb: Reverb::new(sample_rate),
      duck: Duck::new(sample_rate),
      dc_block: DcBlockStereo::new(sample_rate),
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
      limiter: Limiter::new(sample_rate),
    }
  }

  fn get_smoothed_parameters(
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

  fn apply_linear_gain(&self, input: (f32, f32), gain: f32) -> (f32, f32) {
    (input.0 * gain, input.1 * gain)
  }

  fn apply_db_gain(&self, input: (f32, f32), level: f32) -> (f32, f32) {
    let gain = level.dbtoa();
    (input.0 * gain, input.1 * gain)
  }

  fn get_delay_input(&self, input: (f32, f32), channel_mode: i32, input_level: f32) -> (f32, f32) {
    self.apply_db_gain(
      match channel_mode {
        1 => ((input.0 + input.1) * 0.5, 0.),
        _ => input,
      },
      input_level,
    )
  }

  fn read_from_delay_lines(
    &mut self,
    time_left: f32,
    time_right: f32,
    time_mode: i32,
    wow_and_flutter_param: f32,
  ) -> (f32, f32) {
    let wow_and_flutter_time = if wow_and_flutter_param > 0. {
      self.wow_and_flutter.run(wow_and_flutter_param)
    } else {
      0.
    };

    match time_mode {
      0 => {
        let time_left = self.smooth_time_left.run(time_left, 0.333);
        let time_right = self.smooth_time_right.run(time_right, 0.333);

        let delay_out_left = self
          .delay_line_left
          .read(time_left + wow_and_flutter_time, Interpolation::Linear);
        let delay_out_right = self
          .delay_line_right
          .read(time_right + wow_and_flutter_time, Interpolation::Linear);

        (delay_out_left, delay_out_right)
      }
      _ => {
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
  }

  fn apply_filter(
    &mut self,
    input: (f32, f32),
    highpass_freq: f32,
    highpass_res: f32,
    lowpass_freq: f32,
    lowpass_res: f32,
  ) -> (f32, f32) {
    let highpass_filter_out =
      self
        .highpass_filter
        .run(input, highpass_freq, highpass_res, FilterType::Highpass);
    self.lowpass_filter.run(
      highpass_filter_out,
      lowpass_freq,
      lowpass_res,
      FilterType::Lowpass,
    )
  }

  fn apply_saturation(&mut self, input: (f32, f32), average: f32) -> (f32, f32, f32) {
    let factor = self.saturation_enabled.run(
      if average > SATURATION_THRESHOLD {
        1.
      } else {
        0.
      },
      7.,
    );
    let inverted_factor = 1. - factor;
    let saturation_gain_compensation = (1. + SATURATION_THRESHOLD - average).clamp(0.25, 1.);

    (
      input.0.fast_tanh1() * factor + input.0 * inverted_factor,
      input.1.fast_tanh1() * factor + input.1 * inverted_factor,
      saturation_gain_compensation,
    )
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
  ) {
    self
      .delay_line_left
      .write(dry_input.0 + feedback_input.0 * feedback);
    self
      .delay_line_right
      .write(dry_input.1 + feedback_input.1 * feedback);
  }

  fn apply_stereo_amount(&self, input: (f32, f32), stereo: f32) -> (f32, f32) {
    let factor = (1. - stereo) * 0.5;
    let inverted_factor = 1. - factor;

    (
      input.0 * inverted_factor + input.1 * factor,
      input.0 * factor + input.1 * inverted_factor,
    )
  }

  pub fn run(
    &mut self,
    input: (f32, f32),
    input_level: f32,
    channel_mode: i32,
    time_mode: i32,
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
  ) -> (f32, f32) {
    let (
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
    ) = self.get_smoothed_parameters(
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
      hold,
    );

    let delay_input = self.get_delay_input(input, channel_mode, input_level);
    let delay_output =
      self.read_from_delay_lines(time_left, time_right, time_mode, wow_and_flutter);
    let filter_output = self.apply_filter(
      delay_output,
      highpass_freq,
      highpass_res,
      lowpass_freq,
      lowpass_res,
    );
    let average = self.average.run((filter_output.0 + filter_output.1) * 0.5);
    let (saturation_output_left, saturation_output_right, saturation_gain_compensation) =
      self.apply_saturation(filter_output, average);
    let feedback_matrix_output = self.apply_channel_mode(
      (saturation_output_left, saturation_output_right),
      channel_mode,
    );
    let db_block_output = self.dc_block.run(feedback_matrix_output);
    self.write_to_delay_lines(delay_input, db_block_output, feedback);

    let stereo_output = self.apply_stereo_amount(filter_output, stereo);
    let reverb_output = self.reverb.run(
      self.apply_linear_gain(stereo_output, saturation_gain_compensation),
      reverb,
      decay,
    );
    let ducking_output = self.duck.run(reverb_output, input, duck);
    let space_echo_output = self.apply_db_gain(ducking_output, output_level);
    let mix_output = Mix::run(input, space_echo_output, mix);
    if limiter {
      self.limiter.run(mix_output)
    } else {
      mix_output
    }
  }
}
