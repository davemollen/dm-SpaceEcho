extern crate lv2;
extern crate space_echo;
use lv2::prelude::*;
use space_echo::{FloatExt, SmoothParameters, SpaceEcho, MIN_DUCK_THRESHOLD};

#[derive(PortCollection)]
struct Ports {
  input: InputPort<Control>,
  time_link: InputPort<Control>,
  time_left: InputPort<Control>,
  time_right: InputPort<Control>,
  feedback: InputPort<Control>,
  hold: InputPort<Control>,
  wow_and_flutter: InputPort<Control>,
  channel_mode: InputPort<Control>,
  time_mode: InputPort<Control>,
  highpass_freq: InputPort<Control>,
  highpass_res: InputPort<Control>,
  lowpass_freq: InputPort<Control>,
  lowpass_res: InputPort<Control>,
  reverb: InputPort<Control>,
  decay: InputPort<Control>,
  stereo: InputPort<Control>,
  duck: InputPort<Control>,
  output: InputPort<Control>,
  mix: InputPort<Control>,
  limiter: InputPort<Control>,
  input_left: InputPort<Audio>,
  input_right: InputPort<Audio>,
  output_left: OutputPort<Audio>,
  output_right: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-SpaceEcho")]
struct DmSpaceEcho {
  space_echo: SpaceEcho,
  smooth_parameters: SmoothParameters,
}

impl DmSpaceEcho {
  pub fn get_params(
    &self,
    ports: &mut Ports,
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
    let hold = *ports.hold == 1.;
    let wow_and_flutter = *ports.wow_and_flutter * 0.01;

    (
      if hold { 0. } else { (*ports.input).dbtoa() },
      *ports.channel_mode as i32 - 1,
      *ports.time_mode as i32 - 1,
      *ports.time_left,
      if *ports.time_link == 1. {
        *ports.time_left
      } else {
        *ports.time_right
      },
      if hold { 1. } else { *ports.feedback * 0.01 },
      if hold {
        0.
      } else {
        wow_and_flutter * wow_and_flutter * wow_and_flutter
      },
      *ports.highpass_freq,
      *ports.highpass_res * 0.01,
      *ports.lowpass_freq,
      *ports.lowpass_res * 0.01,
      *ports.reverb * 0.01,
      *ports.decay * 0.005,
      *ports.stereo * 0.01,
      (*ports.duck * 0.01 * MIN_DUCK_THRESHOLD).dbtoa(),
      (*ports.output).dbtoa(),
      *ports.mix * 0.01,
      *ports.limiter == 1.,
      if hold { 0. } else { 1. },
    )
  }
}

impl Plugin for DmSpaceEcho {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = plugin_info.sample_rate() as f32;

    Some(Self {
      space_echo: SpaceEcho::new(sample_rate),
      smooth_parameters: SmoothParameters::new(sample_rate)
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
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
    ) = self.get_params(ports);
    self.smooth_parameters.set_targets(input_level, time_left, time_right, feedback, flutter_gain, highpass_freq, lowpass_freq, reverb, decay, stereo, output_level, mix, filter_gain);

    let input_channels = ports.input_left.iter().zip(ports.input_right.iter());
    let output_channels = ports
      .output_left
      .iter_mut()
      .zip(ports.output_right.iter_mut());

    for ((input_left, input_right), (output_left, output_right)) in
      input_channels.zip(output_channels)
    {
      (*output_left, *output_right) = self.space_echo.process(
        (*input_left, *input_right),
        channel_mode,
        time_mode,
        highpass_res,
        lowpass_res,
        duck_threshold,
        limiter,
        &mut self.smooth_parameters,
      );
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmSpaceEcho);
