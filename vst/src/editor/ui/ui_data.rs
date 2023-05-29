use crate::space_echo_parameters::{Params, SpaceEchoParameters};
use std::sync::Arc;
use vizia::{
  prelude::{Event, EventContext, Lens},
  state::{Model, Wrapper},
};
use vst::{host::Host, prelude::HostCallback};

fn notify_host_parameter_changed(index: i32, value: f32, host: Option<HostCallback>) {
  match host {
    Some(host) => {
      host.begin_edit(index);
      host.automate(index, value);
      host.end_edit(index);
    }
    None => {}
  }
}

pub enum ParamChangeEvent {
  SetInput(f32),
  SetChannelMode(i32),
  SetTimeMode(i32),
  SetTimeLink(bool),
  SetTimeLeft(f32),
  SetTimeRight(f32),
  SetFeedback(f32),
  SetWowAndFlutter(f32),
  SetHighpassFreq(f32),
  SetHighpassRes(f32),
  SetLowpassFreq(f32),
  SetLowpassRes(f32),
  SetReverb(f32),
  SetDecay(f32),
  SetStereo(f32),
  SetDuck(f32),
  SetOutput(f32),
  SetMix(f32),
  SetLimiter(bool),
  SetHold(bool),
}

#[derive(Lens)]
pub struct UiData {
  pub params: Arc<SpaceEchoParameters>,
  pub host: Option<HostCallback>,
}

impl Model for UiData {
  fn event(&mut self, _: &mut EventContext, event: &mut Event) {
    event.map(|app_event, _| match app_event {
      ParamChangeEvent::SetInput(value) => {
        let param = &self.params.input;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetChannelMode(value) => {
        let param = &self.params.channel_mode;
        param.set_plain_value(*value);
        notify_host_parameter_changed(
          param.index,
          param.preview_normalized_value(*value),
          self.host,
        );
      }

      ParamChangeEvent::SetTimeMode(value) => {
        let param = &self.params.time_mode;
        param.set_plain_value(*value);
        notify_host_parameter_changed(
          param.index,
          param.preview_normalized_value(*value),
          self.host,
        );
      }

      ParamChangeEvent::SetTimeLink(value) => {
        let param = &self.params.time_link;
        param.set_plain_value(*value);
        notify_host_parameter_changed(
          param.index,
          param.preview_normalized_value(*value),
          self.host,
        );
      }

      ParamChangeEvent::SetTimeLeft(value) => {
        if self.params.time_link.get_value() {
          let param = &self.params.time_right;
          param.set_plain_value(*value);
          notify_host_parameter_changed(param.index, *value, self.host);
        }
        let param = &self.params.time_left;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetTimeRight(value) => {
        if self.params.time_link.get_value() {
          let param = &self.params.time_left;
          param.set_plain_value(*value);
          notify_host_parameter_changed(param.index, *value, self.host);
        }
        let param = &self.params.time_right;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetFeedback(value) => {
        let param = &self.params.feedback;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetWowAndFlutter(value) => {
        let param = &self.params.wow_and_flutter;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetHighpassFreq(value) => {
        let param = &self.params.highpass_freq;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetHighpassRes(value) => {
        let param = &self.params.highpass_res;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetLowpassFreq(value) => {
        let param = &self.params.lowpass_freq;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetLowpassRes(value) => {
        let param = &self.params.lowpass_res;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetReverb(value) => {
        let param = &self.params.reverb;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetDecay(value) => {
        let param = &self.params.decay;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetStereo(value) => {
        let param = &self.params.stereo;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetDuck(value) => {
        let param = &self.params.duck;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetOutput(value) => {
        let param = &self.params.output;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetMix(value) => {
        let param = &self.params.mix;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetLimiter(value) => {
        let param = &self.params.limiter;
        param.set_plain_value(*value);
        notify_host_parameter_changed(
          param.index,
          param.preview_normalized_value(*value),
          self.host,
        );
      }

      ParamChangeEvent::SetHold(value) => {
        let param = &self.params.hold;
        param.set_plain_value(*value);
        notify_host_parameter_changed(
          param.index,
          param.preview_normalized_value(*value),
          self.host,
        );
      }
    });
  }
}
