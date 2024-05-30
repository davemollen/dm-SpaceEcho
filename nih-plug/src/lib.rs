use nih_plug::prelude::*;
use space_echo::{FloatExt, SpaceEcho, MIN_DUCK_THRESHOLD};
mod space_echo_parameters;
use space_echo_parameters::SpaceEchoParameters;
use std::sync::Arc;
mod editor;

struct DmSpaceEcho {
  params: Arc<SpaceEchoParameters>,
  space_echo: SpaceEcho,
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
    let hold = self.params.hold.value();
    let wow_and_flutter = self.params.wow_and_flutter.value();

    (
      if hold {
        0.
      } else {
        self.params.input.value().dbtoa()
      },
      self.params.channel_mode.value() as i32,
      self.params.time_mode.value() as i32,
      self.params.time_left.value(),
      if self.params.time_link.value() {
        self.params.time_left.value()
      } else {
        self.params.time_right.value()
      },
      if hold {
        1.
      } else {
        self.params.feedback.value()
      },
      if hold {
        0.
      } else {
        wow_and_flutter * wow_and_flutter * wow_and_flutter
      },
      self.params.highpass_freq.value(),
      self.params.highpass_res.value(),
      self.params.lowpass_freq.value(),
      self.params.lowpass_res.value(),
      self.params.reverb.value(),
      self.params.decay.value(),
      self.params.stereo.value(),
      (self.params.duck.value() * MIN_DUCK_THRESHOLD).dbtoa(),
      self.params.output.value().dbtoa(),
      self.params.mix.value(),
      self.params.limiter.value(),
      if self.params.hold.value() { 0. } else { 1. },
    )
  }
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
    let (
      input_level,
      _channel_mode,
      _time_mode,
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
      _duck_threshold,
      output_level,
      mix,
      _limiter,
      filter_gain,
    ) = self.get_params();
    self.space_echo.initialize_params_to_smooth(
      input_level,
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
      output_level,
      mix,
      filter_gain,
    );
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
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
