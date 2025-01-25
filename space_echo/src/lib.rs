#![feature(portable_simd)]
mod average;
mod duck;
mod limiter;
mod params;
mod reverb;
mod saturation;
mod tsk_filter_stereo;
mod variable_delay_read;
mod wow_and_flutter;
mod shared {
  pub mod delay_line;
  pub mod delta;
  pub mod float_ext;
  pub mod mix;
  pub mod phasor;
  pub mod random_oscillator;
}

use {
  average::Average,
  duck::Duck,
  limiter::Limiter,
  params::Smoother,
  saturation::Saturation,
  shared::{
    delay_line::{DelayLine, Interpolation},
    float_ext::FloatExt,
    mix::Mix,
  },
  std::simd::{f32x2, num::SimdFloat},
  tsk_filter_stereo::{FilterType, TSKFilterStereo},
  variable_delay_read::VariableDelayRead,
  wow_and_flutter::{WowAndFlutter, MAX_WOW_AND_FLUTTER_TIME_IN_SECS},
};
pub use {params::Params, reverb::Reverb};

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
      average: Average::new(sample_rate, 20.),
      highpass_filter: TSKFilterStereo::new(sample_rate),
      lowpass_filter: TSKFilterStereo::new(sample_rate),
      reverb: Reverb::new(sample_rate),
      duck: Duck::new(sample_rate),
      limiter: Limiter::new(sample_rate, 2., 10., 40., 0.966051),
    }
  }

  pub fn process(&mut self, input: (f32, f32), params: &mut Params) -> (f32, f32) {
    let Params {
      time_mode,
      channel_mode,
      lowpass_res,
      highpass_res,
      duck_threshold,
      limiter,
      ..
    } = *params;
    let input_level = params.input_level.next();
    let feedback = params.feedback.next();
    let flutter_gain = params.flutter_gain.next();
    let highpass_freq = params.highpass_freq.next();
    let lowpass_freq = params.lowpass_freq.next();
    let reverb = params.reverb.next();
    let decay = params.decay.next();
    let stereo = params.stereo.next();
    let output_level = params.output_level.next();
    let mix = params.mix.next();
    let filter_fader = params.filter_fader.next();
    let (time_left, time_right) = params.get_time(time_mode);

    let delay_input = self.get_delay_input(input, channel_mode, input_level);
    let delay_output = self.read_from_delay_lines(time_left, time_right, time_mode, flutter_gain);

    let average = self
      .average
      .process(Self::take_loudest_channel(delay_output));
    let gain_compensation = Self::retrieve_gain_compensation(average, 0.4);

    let filter_output = self.apply_filter(
      delay_output,
      highpass_freq,
      highpass_res,
      lowpass_freq,
      lowpass_res,
      filter_fader,
    );
    let feedback_matrix_output = self.apply_channel_mode(filter_output, channel_mode);
    self.write_to_delay_lines(delay_input, feedback_matrix_output, feedback, average);

    let stereo_output =
      self.apply_stereo_amount(filter_output, stereo) * f32x2::splat(gain_compensation);

    let reverb_output = self
      .reverb
      .process((stereo_output[0], stereo_output[1]), reverb, decay);
    let ducking_output = self.duck.process(reverb_output, input, duck_threshold);
    let space_echo_output = self.apply_gain(ducking_output, output_level);
    let mix_output = Mix::process(input, space_echo_output, mix);
    self.limiter.process(mix_output, limiter)
  }

  fn apply_gain(&self, input: (f32, f32), gain: f32) -> (f32, f32) {
    (input.0 * gain, input.1 * gain)
  }

  fn get_delay_input(&self, input: (f32, f32), channel_mode: i32, gain: f32) -> f32x2 {
    let input = self.apply_gain(input, gain);

    match channel_mode {
      1 => f32x2::from_array([(input.0 + input.1) * 0.5, 0.]),
      _ => f32x2::from_array([input.0, input.1]),
    }
  }

  fn read_from_delay_lines(
    &mut self,
    time_left: f32,
    time_right: f32,
    time_mode: i32,
    flutter_gain: f32,
  ) -> f32x2 {
    let wow_and_flutter_time = if flutter_gain > 0. {
      self.wow_and_flutter.process(flutter_gain)
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

      f32x2::from_array([delay_out_left, delay_out_right])
    } else {
      let delay_out_left = self.variable_delay_read_left.read(
        &self.delay_line_left,
        time_left,
        wow_and_flutter_time,
        Interpolation::Linear,
      );
      let delay_out_right = self.variable_delay_read_right.read(
        &self.delay_line_right,
        time_right,
        wow_and_flutter_time,
        Interpolation::Linear,
      );

      f32x2::from_array([delay_out_left, delay_out_right])
    }
  }

  fn apply_filter(
    &mut self,
    input: f32x2,
    highpass_freq: f32,
    highpass_res: f32,
    lowpass_freq: f32,
    lowpass_res: f32,
    filter_fader: f32,
  ) -> f32x2 {
    match filter_fader {
      0. => input,
      1. => self.get_filter_output(
        input,
        highpass_freq,
        highpass_res,
        lowpass_freq,
        lowpass_res,
      ),
      _ => {
        let filter_out = self.get_filter_output(
          input,
          highpass_freq,
          highpass_res,
          lowpass_freq,
          lowpass_res,
        );
        input + (filter_out - input) * f32x2::splat(filter_fader)
      }
    }
  }

  fn get_filter_output(
    &mut self,
    input: f32x2,
    highpass_freq: f32,
    highpass_res: f32,
    lowpass_freq: f32,
    lowpass_res: f32,
  ) -> f32x2 {
    let highpass_filter_out =
      self
        .highpass_filter
        .process(input, highpass_freq, highpass_res, FilterType::Highpass);
    self.lowpass_filter.process(
      highpass_filter_out,
      lowpass_freq,
      lowpass_res,
      FilterType::Lowpass,
    )
  }

  fn apply_channel_mode(&mut self, input: f32x2, channel_mode: i32) -> f32x2 {
    match channel_mode {
      1 => input.reverse(),
      _ => input,
    }
  }

  fn write_to_delay_lines(
    &mut self,
    dry_input: f32x2,
    feedback_input: f32x2,
    feedback: f32,
    saturation_mix: f32,
  ) {
    let feedback_output = dry_input + feedback_input * f32x2::splat(feedback);
    let saturation_output = Saturation::process(feedback_output, saturation_mix);

    self.delay_line_left.write(saturation_output[0]);
    self.delay_line_right.write(saturation_output[1]);
  }

  fn apply_stereo_amount(&self, input: f32x2, stereo: f32) -> f32x2 {
    let factor = (1. - stereo) * 0.5;

    input + (input - input.reverse()) * f32x2::splat(factor)
  }

  fn take_loudest_channel(input: f32x2) -> f32 {
    input.abs().reduce_max()
  }

  fn retrieve_gain_compensation(average: f32, threshold: f32) -> f32 {
    if average > threshold {
      threshold / average
    } else {
      1.
    }
  }
}
