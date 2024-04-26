use nih_plug::prelude::*;
use space_echo::SpaceEcho;
mod space_echo_parameters;
use space_echo_parameters::SpaceEchoParameters;
use std::sync::Arc;
mod editor;

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
  const URL: &'static str = "https://github.com/davemollen/dm-SpaceEcho";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: NonZeroU32::new(2),
    main_output_channels: NonZeroU32::new(2),
    ..AudioIOLayout::const_default()
  }];
  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any background
  // tasks.
  type BackgroundTask = ();
  type SysExMessage = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    editor::create(self.params.clone(), self.params.editor_state.clone())
  }

  fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
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
    let time_link = self.params.time_link.value();
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
      let channel_iterator = &mut channel_samples.iter_mut();
      let left_channel = channel_iterator.next().unwrap();
      let right_channel = channel_iterator.next().unwrap();

      let (space_echo_left, space_echo_right) = self.space_echo.process(
        (*left_channel, *right_channel),
        input_level,
        channel_mode,
        time_mode,
        time_left,
        time_right,
        time_link,
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
        0.25,
      );

      *left_channel = space_echo_left;
      *right_channel = space_echo_right;
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
    ClapFeature::Delay,
    ClapFeature::Reverb,
  ];
}

impl Vst3Plugin for DmSpaceEcho {
  const VST3_CLASS_ID: [u8; 16] = *b"dm-SpaceEcho....";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
    Vst3SubCategory::Fx,
    Vst3SubCategory::Stereo,
    Vst3SubCategory::Delay,
    Vst3SubCategory::Reverb,
  ];
}

nih_export_clap!(DmSpaceEcho);
nih_export_vst3!(DmSpaceEcho);
