use nih_plug::prelude::*;
use space_echo::{Params as ProcessParams, SpaceEcho};
mod space_echo_parameters;
use space_echo_parameters::SpaceEchoParameters;
use std::sync::Arc;
mod editor;

struct DmSpaceEcho {
  params: Arc<SpaceEchoParameters>,
  space_echo: SpaceEcho,
  process_params: ProcessParams,
}

impl DmSpaceEcho {
  fn get_time_params(&self, context: &mut impl ProcessContext<Self>) -> (f32, f32) {
    let bpm = context.transport().tempo.unwrap_or(120.) as f32;
    let beat_time = 60000. / bpm;

    let time_left = if self.params.sync_left.value() {
      self.get_synced_time(beat_time, self.params.division_left.value())
    } else {
      self.params.time_left.value()
    };

    let time_right = match (
      self.params.time_link.value(),
      self.params.sync_right.value(),
    ) {
      (true, _) => time_left,
      (false, true) => self.get_synced_time(beat_time, self.params.division_right.value()),
      (false, false) => self.params.time_right.value(),
    };

    (time_left, time_right)
  }

  fn get_synced_time(&self, beat_time: f32, division: i32) -> f32 {
    let factor = match division {
      0 => 0.125,
      1 => 0.166666666666667,
      2 => 0.1875,
      3 => 0.25,
      4 => 0.333333333333333,
      5 => 0.375,
      6 => 0.5,
      7 => 0.666666666666667,
      8 => 0.75,
      9 => 1.,
      10 => 1.333333333333333,
      11 => 1.5,
      12 => 2.,
      13 => 2.666666666666667,
      14 => 3.,
      15 => 4.,
      _ => panic!("synced_time value is out of range."),
    };
    beat_time * factor
  }
}

impl Default for DmSpaceEcho {
  fn default() -> Self {
    let params = Arc::new(SpaceEchoParameters::default());
    Self {
      params: params.clone(),
      space_echo: SpaceEcho::new(44100.),
      process_params: ProcessParams::new(44100.),
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
    self.process_params = ProcessParams::new(buffer_config.sample_rate);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let (time_left, time_right) = self.get_time_params(context);
    self.process_params.set(
      self.params.input.value(),
      self.params.channel_mode.value() as i32,
      self.params.time_mode.value() as i32,
      self.params.time_link.value(),
      time_left,
      time_right,
      self.params.feedback.value(),
      self.params.wow_and_flutter.value(),
      self.params.highpass_freq.value(),
      self.params.highpass_res.value(),
      self.params.lowpass_freq.value(),
      self.params.lowpass_res.value(),
      self.params.reverb.value(),
      self.params.decay.value(),
      self.params.stereo.value(),
      self.params.duck.value(),
      self.params.output.value(),
      self.params.mix.value(),
      self.params.limiter.value(),
      self.params.hold.value(),
    );

    buffer.iter_samples().for_each(|mut channel_samples| {
      let channel_iterator = &mut channel_samples.iter_mut();
      let left_channel = channel_iterator.next().unwrap();
      let right_channel = channel_iterator.next().unwrap();

      (*left_channel, *right_channel) = self
        .space_echo
        .process((*left_channel, *right_channel), &mut self.process_params);
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
