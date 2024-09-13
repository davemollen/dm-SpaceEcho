use nih_plug::{
  formatters::{s2v_f32_hz_then_khz, s2v_f32_percentage, v2s_f32_hz_then_khz, v2s_f32_percentage},
  params::{enums::Enum, EnumParam, IntParam},
  prelude::{BoolParam, FloatParam, FloatRange, IntRange, Params},
};
use std::sync::Arc;
mod custom_formatters;
use crate::editor;
use custom_formatters::{s2v_f32_synced_time, v2s_f32_digits, v2s_f32_synced_time};
use nih_plug_vizia::ViziaState;

#[derive(Enum, PartialEq)]
pub enum ChannelMode {
  Stereo,
  #[name = "Ping Pong"]
  PingPong,
}

#[derive(Enum, PartialEq)]
pub enum TimeMode {
  Repitch,
  Fade,
}

#[derive(Params)]
pub struct SpaceEchoParameters {
  #[persist = "editor-state"]
  pub editor_state: Arc<ViziaState>,

  #[id = "input"]
  pub input: FloatParam,

  #[id = "channel_mode"]
  pub channel_mode: EnumParam<ChannelMode>,

  #[id = "time_mode"]
  pub time_mode: EnumParam<TimeMode>,

  #[id = "time_link"]
  pub time_link: BoolParam,

  #[id = "sync_left"]
  pub sync_left: BoolParam,

  #[id = "sync_right"]
  pub sync_right: BoolParam,

  #[id = "time_left"]
  pub time_left: FloatParam,

  #[id = "time_right"]
  pub time_right: FloatParam,

  #[id = "division_left"]
  pub division_left: IntParam,

  #[id = "division_right"]
  pub division_right: IntParam,

  #[id = "feedback"]
  pub feedback: FloatParam,

  #[id = "wow_and_flutter"]
  pub wow_and_flutter: FloatParam,

  #[id = "highpass_freq"]
  pub highpass_freq: FloatParam,

  #[id = "highpass_res"]
  pub highpass_res: FloatParam,

  #[id = "lowpass_freq"]
  pub lowpass_freq: FloatParam,

  #[id = "lowpass_res"]
  pub lowpass_res: FloatParam,

  #[id = "reverb"]
  pub reverb: FloatParam,

  #[id = "decay"]
  pub decay: FloatParam,

  #[id = "stereo"]
  pub stereo: FloatParam,

  #[id = "duck"]
  pub duck: FloatParam,

  #[id = "output"]
  pub output: FloatParam,

  #[id = "mix"]
  pub mix: FloatParam,

  #[id = "limiter"]
  pub limiter: BoolParam,

  #[id = "hold"]
  pub hold: BoolParam,
}

impl Default for SpaceEchoParameters {
  fn default() -> Self {
    Self {
      editor_state: editor::default_state(),

      input: FloatParam::new(
        "Input",
        0.,
        FloatRange::Linear {
          min: -32.,
          max: 32.,
        },
      )
      .with_unit(" dB")
      .with_value_to_string(v2s_f32_digits(2)),

      channel_mode: EnumParam::new("Channel Mode", ChannelMode::Stereo),

      time_mode: EnumParam::new("Time Mode", TimeMode::Repitch),

      sync_left: BoolParam::new("Sync Left", false),

      sync_right: BoolParam::new("Sync Right", false),

      time_link: BoolParam::new("Link", true),

      time_left: FloatParam::new(
        "Time Left",
        250.,
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
        FloatRange::Skewed {
          min: 1.,
          max: 2500.,
          factor: 0.3,
        },
      )
      .with_unit(" ms")
      .with_value_to_string(v2s_f32_digits(2)),

      division_left: IntParam::new("Division Left", 9, IntRange::Linear { min: 0, max: 15 })
        .with_value_to_string(v2s_f32_synced_time())
        .with_string_to_value(s2v_f32_synced_time()),

      division_right: IntParam::new("Division Right", 9, IntRange::Linear { min: 0, max: 15 })
        .with_value_to_string(v2s_f32_synced_time())
        .with_string_to_value(s2v_f32_synced_time()),

      feedback: FloatParam::new("Feedback", 0.5, FloatRange::Linear { min: 0., max: 1.5 })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      wow_and_flutter: FloatParam::new("Flutter", 0., FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      highpass_freq: FloatParam::new(
        "Highpass",
        20.,
        FloatRange::Skewed {
          min: 20.,
          max: 20000.,
          factor: 0.2,
        },
      )
      .with_value_to_string(v2s_f32_hz_then_khz(2))
      .with_string_to_value(s2v_f32_hz_then_khz()),

      highpass_res: FloatParam::new("Res", 0., FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      lowpass_freq: FloatParam::new(
        "Lowpass",
        6000.,
        FloatRange::Skewed {
          min: 20.,
          max: 20000.,
          factor: 0.2,
        },
      )
      .with_value_to_string(v2s_f32_hz_then_khz(2))
      .with_string_to_value(s2v_f32_hz_then_khz()),

      lowpass_res: FloatParam::new("Res", 0., FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      reverb: FloatParam::new("Reverb", 0., FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      decay: FloatParam::new("Decay", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      stereo: FloatParam::new("Stereo", 1., FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      duck: FloatParam::new("Duck", 0., FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      output: FloatParam::new(
        "Output",
        0.,
        FloatRange::SymmetricalSkewed {
          min: -70.,
          max: 12.,
          factor: 0.333333,
          center: 0.,
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

      mix: FloatParam::new("Mix", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      limiter: BoolParam::new("Limiter", false),

      hold: BoolParam::new("Hold", false),
    }
  }
}
