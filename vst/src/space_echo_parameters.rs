use std::sync::Arc;
use vst::plugin::PluginParameters;
mod formatters;
use formatters::{
  s2v_channel_mode, s2v_f32_hz_then_khz, s2v_f32_percentage, s2v_time_mode, v2s_channel_mode,
  v2s_f32_digits, v2s_f32_hz_then_khz, v2s_f32_percentage, v2s_time_mode,
};
mod params;
pub use params::{BoolParam, FloatParam, FloatRange, IntParam, IntRange, Params};

pub struct SpaceEchoParameters {
  pub input: FloatParam,
  pub channel_mode: IntParam,
  pub time_mode: IntParam,
  pub time_link: BoolParam,
  pub time_left: FloatParam,
  pub time_right: FloatParam,
  pub feedback: FloatParam,
  pub wow_and_flutter: FloatParam,
  pub highpass_freq: FloatParam,
  pub highpass_res: FloatParam,
  pub lowpass_freq: FloatParam,
  pub lowpass_res: FloatParam,
  pub reverb: FloatParam,
  pub decay: FloatParam,
  pub stereo: FloatParam,
  pub duck: FloatParam,
  pub output: FloatParam,
  pub mix: FloatParam,
  pub limiter: BoolParam,
  pub hold: BoolParam,
}

impl Default for SpaceEchoParameters {
  fn default() -> Self {
    Self {
      input: FloatParam::new(
        "Input",
        0.,
        0,
        FloatRange::Linear {
          min: -32.,
          max: 32.,
        },
      )
      .with_unit(" dB")
      .with_value_to_string(v2s_f32_digits(2)),

      channel_mode: IntParam::new(
        "Channel Mode",
        0,
        1,
        IntRange::Options {
          names: vec!["Stereo", "Ping Pong"],
        },
      )
      .with_value_to_string(v2s_channel_mode())
      .with_string_to_value(s2v_channel_mode()),

      time_mode: IntParam::new(
        "Time Mode",
        0,
        2,
        IntRange::Options {
          names: vec!["Repitch", "Fade"],
        },
      )
      .with_value_to_string(v2s_time_mode())
      .with_string_to_value(s2v_time_mode()),

      time_link: BoolParam::new("Link", true, 3),

      time_left: FloatParam::new(
        "Time Left",
        250.,
        4,
        FloatRange::Skewed {
          min: 1.,
          max: 2500.,
          factor: 0.3,
        },
      )
      .with_unit(" ms")
      .with_value_to_string(v2s_f32_digits(2)),

      time_right: FloatParam::new(
        "Time Right",
        250.,
        5,
        FloatRange::Skewed {
          min: 1.,
          max: 2500.,
          factor: 0.3,
        },
      )
      .with_unit(" ms")
      .with_value_to_string(v2s_f32_digits(2)),

      feedback: FloatParam::new("Feedback", 0.5, 6, FloatRange::Linear { min: 0., max: 1.5 })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      wow_and_flutter: FloatParam::new("Flutter", 0., 7, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      highpass_freq: FloatParam::new(
        "Highpass",
        20.,
        8,
        FloatRange::Skewed {
          min: 20.,
          max: 20000.,
          factor: 0.2,
        },
      )
      .with_value_to_string(v2s_f32_hz_then_khz(2))
      .with_string_to_value(s2v_f32_hz_then_khz()),

      highpass_res: FloatParam::new("Res", 0., 9, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      lowpass_freq: FloatParam::new(
        "Lowpass",
        6000.,
        10,
        FloatRange::Skewed {
          min: 20.,
          max: 20000.,
          factor: 0.2,
        },
      )
      .with_value_to_string(v2s_f32_hz_then_khz(2))
      .with_string_to_value(s2v_f32_hz_then_khz()),

      lowpass_res: FloatParam::new("Res", 0., 11, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      reverb: FloatParam::new("Reverb", 0., 12, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      decay: FloatParam::new("Decay", 0.5, 13, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      stereo: FloatParam::new("Stereo", 1., 14, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      duck: FloatParam::new("Duck", 0., 15, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      output: FloatParam::new(
        "Output",
        0.,
        16,
        FloatRange::AsymmetricalSkewed {
          min: -70.,
          max: 12.,
          center: 0.,
          below_center_factor: 0.333333,
          above_center_factor: 1.,
        },
      )
      .with_unit(" dB")
      .with_value_to_string(Arc::new(move |value| {
        if value == -70. {
          "-inf".to_string()
        } else {
          format!("{:.2}", value)
        }
      })),

      mix: FloatParam::new("Mix", 0.5, 17, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      limiter: BoolParam::new("Limiter", false, 18),

      hold: BoolParam::new("Hold", false, 19),
    }
  }
}

impl PluginParameters for SpaceEchoParameters {
  fn get_parameter(&self, index: i32) -> f32 {
    match index {
      0 => self.input.get_normalized_value(),
      1 => self.channel_mode.get_normalized_value(),
      2 => self.time_mode.get_normalized_value(),
      3 => self.time_link.get_normalized_value(),
      4 => self.time_left.get_normalized_value(),
      5 => self.time_right.get_normalized_value(),
      6 => self.feedback.get_normalized_value(),
      7 => self.wow_and_flutter.get_normalized_value(),
      8 => self.highpass_freq.get_normalized_value(),
      9 => self.highpass_res.get_normalized_value(),
      10 => self.lowpass_freq.get_normalized_value(),
      11 => self.lowpass_res.get_normalized_value(),
      12 => self.reverb.get_normalized_value(),
      13 => self.decay.get_normalized_value(),
      14 => self.stereo.get_normalized_value(),
      15 => self.duck.get_normalized_value(),
      16 => self.output.get_normalized_value(),
      17 => self.mix.get_normalized_value(),
      18 => self.limiter.get_normalized_value(),
      19 => self.hold.get_normalized_value(),
      _ => 0.,
    }
  }

  fn get_parameter_text(&self, index: i32) -> String {
    match index {
      0 => self.input.get_display_value(true),
      1 => self.channel_mode.get_display_value(true),
      2 => self.time_mode.get_display_value(true),
      3 => self.time_link.get_display_value(true),
      4 => self.time_left.get_display_value(true),
      5 => self.time_right.get_display_value(true),
      6 => self.feedback.get_display_value(true),
      7 => self.wow_and_flutter.get_display_value(true),
      8 => self.highpass_freq.get_display_value(true),
      9 => self.highpass_res.get_display_value(true),
      10 => self.lowpass_freq.get_display_value(true),
      11 => self.lowpass_res.get_display_value(true),
      12 => self.reverb.get_display_value(true),
      13 => self.decay.get_display_value(true),
      14 => self.stereo.get_display_value(true),
      15 => self.duck.get_display_value(true),
      16 => self.output.get_display_value(true),
      17 => self.mix.get_display_value(true),
      18 => self.limiter.get_display_value(true),
      19 => self.hold.get_display_value(true),
      _ => "".to_string(),
    }
  }

  fn get_parameter_name(&self, index: i32) -> String {
    match index {
      0 => self.input.name,
      1 => self.channel_mode.name,
      2 => self.time_mode.name,
      3 => self.time_link.name,
      4 => self.time_left.name,
      5 => self.time_right.name,
      6 => self.feedback.name,
      7 => self.wow_and_flutter.name,
      8 => self.highpass_freq.name,
      9 => self.highpass_res.name,
      10 => self.lowpass_freq.name,
      11 => self.lowpass_res.name,
      12 => self.reverb.name,
      13 => self.decay.name,
      14 => self.stereo.name,
      15 => self.duck.name,
      16 => self.output.name,
      17 => self.mix.name,
      18 => self.limiter.name,
      19 => self.hold.name,
      _ => "",
    }
    .to_string()
  }

  fn set_parameter(&self, index: i32, val: f32) {
    match index {
      0 => self.input.set_plain_value(val),
      1 => self.channel_mode.set_normalized_value(val),
      2 => self.time_mode.set_normalized_value(val),
      3 => self.time_link.set_normalized_value(val),
      4 => self.time_left.set_plain_value(val),
      5 => self.time_right.set_plain_value(val),
      6 => self.feedback.set_plain_value(val),
      7 => self.wow_and_flutter.set_plain_value(val),
      8 => self.highpass_freq.set_plain_value(val),
      9 => self.highpass_res.set_plain_value(val),
      10 => self.lowpass_freq.set_plain_value(val),
      11 => self.lowpass_res.set_plain_value(val),
      12 => self.reverb.set_plain_value(val),
      13 => self.decay.set_plain_value(val),
      14 => self.stereo.set_plain_value(val),
      15 => self.duck.set_plain_value(val),
      16 => self.output.set_plain_value(val),
      17 => self.mix.set_plain_value(val),
      18 => self.limiter.set_normalized_value(val),
      19 => self.hold.set_normalized_value(val),
      _ => (),
    }
  }
}
