#[macro_use]
extern crate vst;
mod editor;
use editor::SpaceEchoEditor;
mod space_echo_parameters;
use space_echo::{FloatExt, SpaceEcho, MIN_DUCK_THRESHOLD};
use space_echo_parameters::{Params, SpaceEchoParameters};
use std::sync::Arc;
use vst::{
  buffer::AudioBuffer,
  editor::Editor,
  plugin::{Category, Info, Plugin, PluginParameters},
  prelude::HostCallback,
};

struct DmSpaceEcho {
  params: Arc<SpaceEchoParameters>,
  space_echo: SpaceEcho,
  editor: Option<SpaceEchoEditor>,
  is_active: bool,
}

impl DmSpaceEcho {
  fn get_params(
    &self,
  ) -> (
    f32,
    i32,
    i32,
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
    bool,
    f32,
  ) {
    let hold = self.params.hold.get_value();
    let wow_and_flutter = self.params.wow_and_flutter.get_value();

    (
      if hold {
        0.
      } else {
        self.params.input.get_value().dbtoa()
      },
      self.params.channel_mode.get_value(),
      self.params.time_mode.get_value(),
      self.params.time_left.get_value(),
      if self.params.time_link.get_value() {
        self.params.time_left.get_value()
      } else {
        self.params.time_right.get_value()
      },
      if hold {
        1.
      } else {
        self.params.feedback.get_value()
      },
      if hold {
        0.
      } else {
        wow_and_flutter * wow_and_flutter * wow_and_flutter
      },
      self.params.highpass_freq.get_value(),
      self.params.highpass_res.get_value(),
      self.params.lowpass_freq.get_value(),
      self.params.lowpass_res.get_value(),
      self.params.reverb.get_value(),
      self.params.decay.get_value(),
      self.params.stereo.get_value(),
      (self.params.duck.get_value() * MIN_DUCK_THRESHOLD).dbtoa(),
      self.params.output.get_value().dbtoa(),
      self.params.mix.get_value(),
      self.params.limiter.get_value(),
      if self.params.hold.get_value() { 0. } else { 1. },
    )
  }
}

impl Plugin for DmSpaceEcho {
  fn new(host: HostCallback) -> Self {
    let params = Arc::new(SpaceEchoParameters::default());

    Self {
      params: params.clone(),
      space_echo: SpaceEcho::new(44100.),
      editor: Some(SpaceEchoEditor {
        params: params.clone(),
        is_open: false,
        host: Some(host),
      }),
      is_active: false,
    }
  }

  fn set_sample_rate(&mut self, sample_rate: f32) {
    self.space_echo = SpaceEcho::new(sample_rate);
  }

  fn get_info(&self) -> Info {
    Info {
      name: "dm-SpaceEcho".to_string(),
      vendor: "DM".to_string(),
      version: 1,
      inputs: 2,
      outputs: 2,
      parameters: 20,
      unique_id: 1358,
      f64_precision: true,
      category: Category::Effect,
      ..Default::default()
    }
  }

  fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
    let (
      input_level,
      channel_mode,
      time_mode,
      time_left,
      time_right,
      feedback,
      flutter_gain,
      highpass_freq,
      highpass_res,
      lowpass_freq,
      lowpass_res,
      reverb,
      decay,
      stereo,
      duck_threshold,
      output_level,
      mix,
      limiter,
      filter_gain,
    ) = self.get_params();

    if !self.is_active {
      self.space_echo.initialize_params_to_smooth(
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
      self.is_active = true;
    }

    let (input_channels, mut output_channels) = buffer.split();
    let zipped_input_channels = input_channels.get(0).iter().zip(input_channels.get(1));
    let zipped_output_channels = output_channels
      .get_mut(0)
      .iter_mut()
      .zip(output_channels.get_mut(1));
    for ((input_left, input_right), (output_left, output_right)) in
      zipped_input_channels.zip(zipped_output_channels)
    {
      let (space_echo_left, space_echo_right) = self.space_echo.process(
        (*input_left, *input_right),
        input_level,
        channel_mode,
        time_mode,
        time_left,
        time_right,
        feedback,
        flutter_gain,
        highpass_freq,
        highpass_res,
        lowpass_freq,
        lowpass_res,
        reverb,
        decay,
        stereo,
        duck_threshold,
        output_level,
        mix,
        limiter,
        filter_gain,
      );
      *output_left = space_echo_left;
      *output_right = space_echo_right;
    }
  }

  fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
    Arc::clone(&self.params) as Arc<dyn PluginParameters>
  }

  fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
    if let Some(editor) = self.editor.take() {
      Some(Box::new(editor) as Box<dyn Editor>)
    } else {
      None
    }
  }
}

plugin_main!(DmSpaceEcho);
