use crate::space_echo_parameters::SpaceEchoParameters;
use std::sync::Arc;
use vizia::{
  prelude::{
    Context, LayoutModifiers, StyleModifiers,
    Units::{Percentage, Pixels, Stretch},
  },
  state::Model,
  style::Color,
  views::{HStack, Label, VStack, ZStack},
};
use vst::prelude::HostCallback;
mod param_knob;
pub use param_knob::ParamKnob;
mod param_checkbox;
pub use param_checkbox::ParamCheckbox;
mod param_radio_button;
pub use param_radio_button::ParamRadioButton;
mod ui_data;
pub use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("./ui/style.css");

pub fn plugin_gui(cx: &mut Context, params: Arc<SpaceEchoParameters>, host: Option<HostCallback>) {
  cx.add_theme(STYLE);

  UiData {
    params: params.clone(),
    host,
  }
  .build(cx);

  ZStack::new(cx, |cx| {
    HStack::new(cx, |cx| {
      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          UiData::params,
          &params.input,
          |params| &params.input,
          |val| ParamChangeEvent::SetInput(val),
        );
      })
      .child_space(Stretch(1.0))
      .row_between(Pixels(10.0));

      VStack::new(cx, |cx| {
        ParamRadioButton::new(
          cx,
          UiData::params,
          &params.channel_mode,
          |params| &params.channel_mode,
          |val| ParamChangeEvent::SetChannelMode(val),
        );
        ParamRadioButton::new(
          cx,
          UiData::params,
          &params.time_mode,
          |params| &params.time_mode,
          |val| ParamChangeEvent::SetTimeMode(val),
        );
      })
      .child_space(Stretch(1.0))
      .row_between(Pixels(10.0));

      VStack::new(cx, |cx| {
        ParamCheckbox::new(
          cx,
          UiData::params,
          &params.time_link,
          |params| &params.time_link,
          |val| ParamChangeEvent::SetTimeLink(val),
        );
      })
      .child_space(Stretch(1.0))
      .row_between(Pixels(10.0));

      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          UiData::params,
          &params.time_left,
          |params| &params.time_left,
          |val| ParamChangeEvent::SetTimeLeft(val),
        );
        ParamKnob::new(
          cx,
          UiData::params,
          &params.time_right,
          |params| &params.time_right,
          |val| ParamChangeEvent::SetTimeRight(val),
        );
      })
      .child_space(Stretch(1.0))
      .row_between(Pixels(10.0));

      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          UiData::params,
          &params.highpass_freq,
          |params| &params.highpass_freq,
          |val| ParamChangeEvent::SetHighpassFreq(val),
        );
        ParamKnob::new(
          cx,
          UiData::params,
          &params.highpass_res,
          |params| &params.highpass_res,
          |val| ParamChangeEvent::SetHighpassRes(val),
        );
      })
      .child_space(Stretch(1.0))
      .row_between(Pixels(10.0));

      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          UiData::params,
          &params.lowpass_freq,
          |params| &params.lowpass_freq,
          |val| ParamChangeEvent::SetLowpassFreq(val),
        );
        ParamKnob::new(
          cx,
          UiData::params,
          &params.lowpass_res,
          |params| &params.lowpass_res,
          |val| ParamChangeEvent::SetLowpassRes(val),
        );
      })
      .child_space(Stretch(1.0))
      .row_between(Pixels(10.0));

      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          UiData::params,
          &params.feedback,
          |params| &params.feedback,
          |val| ParamChangeEvent::SetFeedback(val),
        );
        ParamKnob::new(
          cx,
          UiData::params,
          &params.wow_and_flutter,
          |params| &params.wow_and_flutter,
          |val| ParamChangeEvent::SetWowAndFlutter(val),
        );
      })
      .child_space(Stretch(1.0))
      .row_between(Pixels(10.0));

      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          UiData::params,
          &params.reverb,
          |params| &params.reverb,
          |val| ParamChangeEvent::SetReverb(val),
        );
        ParamKnob::new(
          cx,
          UiData::params,
          &params.decay,
          |params| &params.decay,
          |val| ParamChangeEvent::SetDecay(val),
        );
      })
      .top(Pixels(-32.0))
      .child_space(Stretch(1.0))
      .row_between(Pixels(10.0));

      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          UiData::params,
          &params.stereo,
          |params| &params.stereo,
          |val| ParamChangeEvent::SetStereo(val),
        );
        ParamKnob::new(
          cx,
          UiData::params,
          &params.duck,
          |params| &params.duck,
          |val| ParamChangeEvent::SetDuck(val),
        );
      })
      .top(Pixels(-64.0))
      .child_space(Stretch(1.0))
      .row_between(Pixels(10.0));

      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          UiData::params,
          &params.output,
          |params| &params.output,
          |val| ParamChangeEvent::SetOutput(val),
        );
        ParamKnob::new(
          cx,
          UiData::params,
          &params.mix,
          |params| &params.mix,
          |val| ParamChangeEvent::SetMix(val),
        );
        ParamCheckbox::new(
          cx,
          UiData::params,
          &params.limiter,
          |params| &params.limiter,
          |val| ParamChangeEvent::SetLimiter(val),
        );
      })
      .top(Pixels(-64.0))
      .child_space(Stretch(1.0))
      .row_between(Pixels(10.0));
    })
    .background_color(Color::rgb(80, 80, 80));

    Label::new(cx, "dm-SpaceEcho")
      .class("plugin-name")
      .top(Percentage(82.0))
      .left(Percentage(66.66));
  });
}
