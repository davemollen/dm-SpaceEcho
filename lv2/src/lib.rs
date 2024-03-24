extern crate lv2;
extern crate space_echo;
use lv2::prelude::*;
use space_echo::SpaceEcho;

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
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let input_level = *ports.input;
    let time_link = *ports.time_link == 1.;
    let time_left = *ports.time_left;
    let time_right = *ports.time_right;
    let feedback = *ports.feedback * 0.01;
    let hold = *ports.hold == 1.;
    let wow_and_flutter = *ports.wow_and_flutter * 0.01;
    let time_mode = *ports.time_mode as i32 - 1;
    let channel_mode = *ports.channel_mode as i32 - 1;
    let highpass_freq = *ports.highpass_freq;
    let highpass_res = *ports.highpass_res * 0.01;
    let lowpass_freq = *ports.lowpass_freq;
    let lowpass_res = *ports.lowpass_res * 0.01;
    let reverb = *ports.reverb * 0.01;
    let decay = *ports.decay * 0.01;
    let stereo = *ports.stereo * 0.01;
    let duck = *ports.duck * 0.01;
    let output_level = *ports.output;
    let mix = *ports.mix * 0.01;
    let limiter = *ports.limiter == 1.;

    let input_channels = ports.input_left.iter().zip(ports.input_right.iter());
    let output_channels = ports
      .output_left
      .iter_mut()
      .zip(ports.output_right.iter_mut());

    for ((input_left, input_right), (output_left, output_right)) in
      input_channels.zip(output_channels)
    {
      let (space_echo_left, space_echo_right) = self.space_echo.run(
        (*input_left, *input_right),
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
        0.1,
      );
      *output_left = space_echo_left;
      *output_right = space_echo_right;
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmSpaceEcho);
