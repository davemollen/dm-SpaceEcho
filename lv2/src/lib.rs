extern crate lv2;
extern crate space_echo;
use lv2::prelude::*;
use space_echo::{Params, SpaceEcho};

#[derive(PortCollection)]
struct Ports {
  input: InputPort<InPlaceControl>,
  time_link: InputPort<InPlaceControl>,
  time_left: InputPort<InPlaceControl>,
  time_right: InputPort<InPlaceControl>,
  feedback: InputPort<InPlaceControl>,
  hold: InputPort<InPlaceControl>,
  wow_and_flutter: InputPort<InPlaceControl>,
  channel_mode: InputPort<InPlaceControl>,
  time_mode: InputPort<InPlaceControl>,
  highpass_freq: InputPort<InPlaceControl>,
  highpass_res: InputPort<InPlaceControl>,
  lowpass_freq: InputPort<InPlaceControl>,
  lowpass_res: InputPort<InPlaceControl>,
  reverb: InputPort<InPlaceControl>,
  decay: InputPort<InPlaceControl>,
  stereo: InputPort<InPlaceControl>,
  duck: InputPort<InPlaceControl>,
  output: InputPort<InPlaceControl>,
  mix: InputPort<InPlaceControl>,
  limiter: InputPort<InPlaceControl>,
  input_left: InputPort<InPlaceAudio>,
  input_right: InputPort<InPlaceAudio>,
  output_left: OutputPort<InPlaceAudio>,
  output_right: OutputPort<InPlaceAudio>,
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
      ports.input.get(),
      ports.channel_mode.get() as i32 - 1,
      ports.time_mode.get() as i32 - 1,
      ports.time_link.get() == 1.,
      ports.time_left.get(),
      ports.time_right.get(),
      ports.feedback.get() * 0.01,
      ports.wow_and_flutter.get() * 0.01,
      ports.highpass_freq.get(),
      ports.highpass_res.get() * 0.01,
      ports.lowpass_freq.get(),
      ports.lowpass_res.get() * 0.01,
      ports.reverb.get() * 0.01,
      ports.decay.get() * 0.005,
      ports.stereo.get() * 0.01,
      ports.duck.get() * 0.01,
      ports.output.get(),
      ports.mix.get() * 0.01,
      ports.limiter.get() == 1.,
      ports.hold.get() == 1.,
    );

    let input_channels = ports.input_left.iter().zip(ports.input_right.iter());
    let output_channels = ports.output_left.iter().zip(ports.output_right.iter());

    for ((input_left, input_right), (output_left, output_right)) in
      input_channels.zip(output_channels)
    {
      let output = self
        .space_echo
        .process((input_left.get(), input_right.get()), &mut self.params);
      output_left.set(output.0);
      output_right.set(output.1);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmSpaceEcho);
