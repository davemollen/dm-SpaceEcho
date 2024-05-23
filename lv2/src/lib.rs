extern crate lv2;
extern crate space_echo;
use lv2::prelude::*;
use space_echo::{FloatExt, MappedParams, SpaceEcho, MIN_DUCK_THRESHOLD};

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
  is_active: bool,
}

impl DmSpaceEcho {
  pub fn get_params(&self, ports: &mut Ports) -> MappedParams {
    let hold = *ports.hold == 1.;
    let wow_and_flutter = *ports.wow_and_flutter * 0.01;

    MappedParams {
      input_level: if hold { 0. } else { (*ports.input).dbtoa() },
      // TODO: limit time_mode and channel_mode to range from 0 to 1
      channel_mode: *ports.channel_mode as i32 - 1,
      time_mode: *ports.time_mode as i32 - 1,
      time_left: *ports.time_left,
      time_right: if *ports.time_link == 1. {
        *ports.time_left
      } else {
        *ports.time_right
      },
      feedback: if hold { 1. } else { *ports.feedback * 0.01 },
      flutter_gain: if hold {
        0.
      } else {
        wow_and_flutter * wow_and_flutter * wow_and_flutter
      },
      highpass_freq: if hold { 20. } else { *ports.highpass_freq },
      highpass_res: if hold { 0. } else { *ports.highpass_res * 0.01 },
      lowpass_freq: if hold { 20000. } else { *ports.lowpass_freq },
      lowpass_res: if hold { 0. } else { *ports.lowpass_res * 0.01 },
      reverb: *ports.reverb * 0.01,
      decay: *ports.decay * 0.01,
      stereo: *ports.stereo * 0.01,
      duck_threshold: (*ports.duck * 0.01 * MIN_DUCK_THRESHOLD).dbtoa(),
      output_level: (*ports.output).dbtoa(),
      mix: *ports.mix * 0.01,
      limiter: *ports.limiter == 1.,
      filter_gain: if hold { 0. } else { 1. },
    }
  }
}

impl Plugin for DmSpaceEcho {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      space_echo: SpaceEcho::new(_plugin_info.sample_rate() as f32),
      is_active: false,
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let params = self.get_params(ports);

    if !self.is_active {
      self.space_echo.initialize_params_to_smooth(&params);
      self.is_active = true;
    }

    let input_channels = ports.input_left.iter().zip(ports.input_right.iter());
    let output_channels = ports
      .output_left
      .iter_mut()
      .zip(ports.output_right.iter_mut());

    for ((input_left, input_right), (output_left, output_right)) in
      input_channels.zip(output_channels)
    {
      let (space_echo_left, space_echo_right) = self
        .space_echo
        .process((*input_left, *input_right), &params);
      *output_left = space_echo_left;
      *output_right = space_echo_right;
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmSpaceEcho);
