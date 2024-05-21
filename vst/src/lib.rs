#[macro_use]
extern crate vst;
mod editor;
use editor::SpaceEchoEditor;
mod space_echo_parameters;
use space_echo::{InputParams, MappedParams, SpaceEcho};
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
  fn get_params(&self) -> MappedParams {
    let params = InputParams {
      input: self.params.input.get_value(),
      channel_mode: self.params.channel_mode.get_value(),
      time_mode: self.params.time_mode.get_value(),
      time_link: self.params.time_link.get_value(),
      time_left: self.params.time_left.get_value(),
      time_right: self.params.time_right.get_value(),
      feedback: self.params.feedback.get_value(),
      wow_and_flutter: self.params.wow_and_flutter.get_value(),
      highpass_freq: self.params.highpass_freq.get_value(),
      highpass_res: self.params.highpass_res.get_value(),
      lowpass_freq: self.params.lowpass_freq.get_value(),
      lowpass_res: self.params.lowpass_res.get_value(),
      reverb: self.params.reverb.get_value(),
      decay: self.params.decay.get_value(),
      stereo: self.params.stereo.get_value(),
      duck: self.params.duck.get_value(),
      output: self.params.output.get_value(),
      mix: self.params.mix.get_value(),
      limiter: self.params.limiter.get_value(),
      hold: self.params.hold.get_value(),
    };

    self.space_echo.map_params(&params)
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
    let params = self.get_params();

    if !self.is_active {
      self.space_echo.initialize_params_to_smooth(&params);
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
      let (space_echo_left, space_echo_right) = self
        .space_echo
        .process((*input_left, *input_right), &params);
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
