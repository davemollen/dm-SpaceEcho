extern crate lv2;
extern crate space_echo;
use lv2::prelude::*;
use space_echo::{Params, SpaceEcho};

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
  params: Params,
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
      params: Params::new(sample_rate),
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    self.params.set(
      *ports.input,
      *ports.channel_mode as i32 - 1,
      *ports.time_mode as i32 - 1,
      *ports.time_link == 1.,
      *ports.time_left,
      *ports.time_right,
      *ports.feedback * 0.01,
      *ports.wow_and_flutter * 0.01,
      *ports.highpass_freq,
      *ports.highpass_res * 0.01,
      *ports.lowpass_freq,
      *ports.lowpass_res * 0.01,
      *ports.reverb * 0.01,
      *ports.decay * 0.005,
      *ports.stereo * 0.01,
      *ports.duck * 0.01,
      *ports.output,
      *ports.mix * 0.01,
      *ports.limiter == 1.,
      *ports.hold == 1.,
    );

    let input_channels = ports.input_left.iter().zip(ports.input_right.iter());
    let output_channels = ports
      .output_left
      .iter_mut()
      .zip(ports.output_right.iter_mut());

    for ((input_left, input_right), (output_left, output_right)) in
      input_channels.zip(output_channels)
    {
      (*output_left, *output_right) = self
        .space_echo
        .process((*input_left, *input_right), &mut self.params);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmSpaceEcho);
