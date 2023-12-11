use nih_plug::prelude::*;
use space_echo::SpaceEcho;
mod space_echo_parameters;
use space_echo_parameters::SpaceEchoParameters;
use std::sync::{atomic::AtomicBool, Arc};
mod editor;
use editor::SpaceEchoEditor;

const TIME_SMOOTHING_FACTOR: f32 = 0.25;

struct DmSpaceEcho {
  params: Arc<SpaceEchoParameters>,
  space_echo: SpaceEcho,
}

impl Default for DmSpaceEcho {
  fn default() -> Self {
    let params = Arc::new(SpaceEchoParameters::default());
    Self {
      params: params.clone(),
      space_echo: SpaceEcho::new(44100.),
    }
  }
}

impl Plugin for DmSpaceEcho {
  const NAME: &'static str = "dm-SpaceEcho";
  const VENDOR: &'static str = "DM";
  // You can use `env!("CARGO_PKG_HOMEPAGE")` to reference the homepage field from the
  // `Cargo.toml` file here
  const URL: &'static str = "https://github.com/davemollen/dm-SpaceEcho";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");
  const DEFAULT_INPUT_CHANNELS: u32 = 2;
  const DEFAULT_OUTPUT_CHANNELS: u32 = 2;
  const DEFAULT_AUX_INPUTS: Option<AuxiliaryIOConfig> = None;
  const DEFAULT_AUX_OUTPUTS: Option<AuxiliaryIOConfig> = None;
  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any background
  // tasks.
  type BackgroundTask = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn accepts_bus_config(&self, config: &BusConfig) -> bool {
    // This works with any symmetrical IO layout
    config.num_input_channels == config.num_output_channels && config.num_input_channels > 0
  }

  fn editor(&self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    Some(Box::new(SpaceEchoEditor {
      params: self.params.clone(),
      emit_parameters_changed_event: Arc::new(AtomicBool::new(false)),
    }))
  }

  fn initialize(
    &mut self,
    _: &BusConfig,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    self.space_echo = SpaceEcho::new(buffer_config.sample_rate);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let input_level = self.params.input.value();
    let time_left = self.params.time_left.value();
    let time_right = self.params.time_right.value();
    let feedback = self.params.feedback.value();
    let wow_and_flutter = self.params.wow_and_flutter.value();
    let highpass_freq = self.params.highpass_freq.value();
    let highpass_res = self.params.highpass_res.value();
    let lowpass_freq = self.params.lowpass_freq.value();
    let lowpass_res = self.params.lowpass_res.value();
    let reverb = self.params.reverb.value();
    let decay = self.params.decay.value();
    let output_level = self.params.output.value();
    let stereo = self.params.stereo.value();
    let duck = self.params.duck.value();
    let mix = self.params.mix.value();
    let channel_mode = self.params.channel_mode.value() as i32;
    let time_mode = self.params.time_mode.value() as i32;
    let limiter = self.params.limiter.value();
    let hold = self.params.hold.value();

    buffer.iter_samples().for_each(|mut channel_samples| {
      let left_channel_in = channel_samples.get_mut(0).unwrap();
      let input_left = *left_channel_in;
      let right_channel_in = channel_samples.get_mut(1).unwrap();
      let input_right = *right_channel_in;

      let (space_echo_left, space_echo_right) = self.space_echo.run(
        (input_left, input_right),
        input_level,
        channel_mode,
        time_mode,
        time_left,
        time_right,
        feedback,
        wow_and_flutter,
        highpass_freq,
        highpass_res,
        lowpass_freq,
        lowpass_res,
        reverb,
        decay,
        stereo,
        duck,
        output_level,
        mix,
        limiter,
        hold,
        TIME_SMOOTHING_FACTOR
      );

      let left_channel_out = channel_samples.get_mut(0).unwrap();
      *left_channel_out = space_echo_left;
      let right_channel_out = channel_samples.get_mut(1).unwrap();
      *right_channel_out = space_echo_right;
    });
    ProcessStatus::Normal
  }

  // This can be used for cleaning up special resources like socket connections whenever the
  // plugin is deactivated. Most plugins won't need to do anything here.
  fn deactivate(&mut self) {}
}

impl ClapPlugin for DmSpaceEcho {
  const CLAP_ID: &'static str = "dm-SpaceEcho";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("A delay plugin");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::AudioEffect,
    ClapFeature::Stereo,
    ClapFeature::Mono,
    ClapFeature::Utility,
  ];
}

impl Vst3Plugin for DmSpaceEcho {
  const VST3_CLASS_ID: [u8; 16] = *b"DM-SpaceEchoPlug";
  const VST3_CATEGORIES: &'static str = "Fx|Delay";
}

nih_export_clap!(DmSpaceEcho);
nih_export_vst3!(DmSpaceEcho);
