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
  pub mod one_pole_filter;
  pub mod param_filter;
  pub mod phasor;
  pub mod random_oscillator;
  pub mod slide;
}

pub use reverb::Reverb;
use {
  dc_block_stereo::DcBlockStereo,
  duck::{Duck, MIN_DUCK_THRESHOLD},
  limiter::Limiter,
  saturation::Saturation,
  shared::{
    delay_line::{DelayLine, Interpolation},
    float_ext::FloatExt,
    mix::Mix,
  },
  smooth_parameters::{SmoothParameters, SmoothedParams},
  tsk_filter_stereo::{FilterType, TSKFilterStereo},
  variable_delay_read::VariableDelayRead,
  wow_and_flutter::{WowAndFlutter, MAX_WOW_AND_FLUTTER_TIME_IN_SECS},
};

pub struct InputParams {
  pub input: f32,
  pub channel_mode: i32,
  pub time_mode: i32,
  pub time_link: bool,
  pub time_left: f32,
  pub time_right: f32,
  pub feedback: f32,
  pub wow_and_flutter: f32,
  pub highpass_freq: f32,
  pub highpass_res: f32,
  pub lowpass_freq: f32,
  pub lowpass_res: f32,
  pub reverb: f32,
  pub decay: f32,
  pub stereo: f32,
  pub duck: f32,
  pub output: f32,
  pub mix: f32,
  pub limiter: bool,
  pub hold: bool,
}

pub struct MappedParams {
  pub input_level: f32,
  pub channel_mode: i32,
  pub time_mode: i32,
  pub time_left: f32,
  pub time_right: f32,
  pub feedback: f32,
  pub flutter_gain: f32,
  pub highpass_freq: f32,
  pub highpass_res: f32,
  pub lowpass_freq: f32,
  pub lowpass_res: f32,
  pub reverb: f32,
  pub decay: f32,
  pub stereo: f32,
  pub duck_threshold: f32,
  pub output_level: f32,
  pub mix: f32,
  pub limiter: bool,
}

pub struct SpaceEcho {
  delay_line_left: DelayLine,
  delay_line_right: DelayLine,
  variable_delay_read_left: VariableDelayRead,
  variable_delay_read_right: VariableDelayRead,
  wow_and_flutter: WowAndFlutter,
  saturation: Saturation,
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
      saturation: Saturation::new(sample_rate),
      highpass_filter: TSKFilterStereo::new(sample_rate),
      lowpass_filter: TSKFilterStereo::new(sample_rate),
      reverb: Reverb::new(sample_rate),
      duck: Duck::new(sample_rate),
      dc_block: DcBlockStereo::new(sample_rate),
      limiter: Limiter::new(sample_rate, 2., 10., 40., 0.966051),
      smooth_parameters: SmoothParameters::new(sample_rate),
    }
  }

  pub fn map_params(&self, params: &InputParams) -> MappedParams {
    MappedParams {
      input_level: if params.hold {
        0.
      } else {
        params.input.dbtoa()
      },
      channel_mode: params.channel_mode,
      time_mode: params.time_mode,
      time_left: params.time_left,
      time_right: if params.time_link {
        params.time_left
      } else {
        params.time_right
      },
      feedback: if params.hold { 1. } else { params.feedback },
      flutter_gain: if params.hold {
        0.
      } else {
        params.wow_and_flutter * params.wow_and_flutter * params.wow_and_flutter
      },
      highpass_freq: if params.hold {
        20.
      } else {
        params.highpass_freq
      },
      highpass_res: if params.hold { 0. } else { params.highpass_res },
      lowpass_freq: if params.hold {
        20000.
      } else {
        params.lowpass_freq
      },
      lowpass_res: if params.hold { 0. } else { params.lowpass_res },
      reverb: params.reverb,
      decay: params.decay,
      stereo: params.stereo,
      duck_threshold: (params.duck * MIN_DUCK_THRESHOLD).dbtoa(),
      output_level: params.output.dbtoa(),
      mix: params.mix,
      limiter: params.limiter,
    }
  }

  pub fn initialize_params_to_smooth(&mut self, mapped_params: &MappedParams) {
    self.smooth_parameters.initialize(mapped_params);
  }

  pub fn process(&mut self, input: (f32, f32), params: &MappedParams) -> (f32, f32) {
    let SmoothedParams {
      input_level,
      feedback,
      wow_gain,
      flutter_gain,
      highpass_freq,
      highpass_res,
      lowpass_freq,
      lowpass_res,
      reverb,
      decay,
      stereo,
      output_level,
      mix,
      time_left,
      time_right,
    } = self.smooth_parameters.get_params(params);

    let delay_input = self.get_delay_input(input, params.channel_mode, input_level);
    let delay_output = self.read_from_delay_lines(
      time_left,
      time_right,
      params.time_mode,
      wow_gain,
      flutter_gain,
    );

    let (saturation_output_left, saturation_output_right, saturation_gain_compensation) =
      self.saturation.process(delay_output, 0.15);

    let filter_output = self.apply_filter(
      (saturation_output_left, saturation_output_right),
      highpass_freq,
      highpass_res,
      lowpass_freq,
      lowpass_res,
    );
    let feedback_matrix_output = self.apply_channel_mode(filter_output, params.channel_mode);
    let db_block_output = self.dc_block.process(feedback_matrix_output);
    self.write_to_delay_lines(delay_input, db_block_output, feedback);

    let stereo_output = self.apply_stereo_amount(filter_output, stereo);
    let reverb_output = self.reverb.process(
      self.apply_gain(stereo_output, saturation_gain_compensation),
      reverb,
      decay,
    );
    let ducking_output = self
      .duck
      .process(reverb_output, input, params.duck_threshold);
    let space_echo_output = self.apply_gain(ducking_output, output_level);
    let mix_output = Mix::process(input, space_echo_output, mix);
    self.limiter.process(mix_output, params.limiter)
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
  ) -> (f32, f32) {
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
}
