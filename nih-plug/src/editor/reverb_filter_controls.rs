#[path = "./components/param_knob.rs"]
mod param_knob;
use super::{ParamChangeEvent, UiData};
use crate::space_echo_parameters::SpaceEchoParameters;
use nih_plug::params::Param;
use nih_plug_vizia::vizia::{
  layout::Units::{Auto, Stretch},
  prelude::{Context, LayoutModifiers, StyleModifiers, Units::Pixels},
  view::Handle,
  views::{HStack, VStack},
};
use param_knob::{ParamKnob, ParamKnobSize};
use std::sync::Arc;

pub fn build(cx: &mut Context, params: Arc<SpaceEchoParameters>) -> Handle<VStack> {
  VStack::new(cx, |cx| {
    HStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        params.reverb.name(),
        UiData::params,
        params.reverb.as_ptr(),
        |params| &params.reverb,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
        ParamKnobSize::Regular,
      );
      ParamKnob::new(
        cx,
        params.decay.name(),
        UiData::params,
        params.decay.as_ptr(),
        |params| &params.decay,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
        ParamKnobSize::Regular,
      );
    })
    .size(Auto)
    .child_space(Pixels(4.0))
    .child_bottom(Pixels(0.0))
    .background_color("#2d5f4f")
    .border_top_left_radius(Pixels(8.0))
    .border_top_right_radius(Pixels(0.0));

    HStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        params.highpass_freq.name(),
        UiData::params,
        params.highpass_freq.as_ptr(),
        |params| &params.highpass_freq,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
        ParamKnobSize::Regular,
      );
      ParamKnob::new(
        cx,
        params.lowpass_freq.name(),
        UiData::params,
        params.lowpass_freq.as_ptr(),
        |params| &params.lowpass_freq,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
        ParamKnobSize::Regular,
      );
    })
    .size(Auto)
    .child_space(Pixels(4.0))
    .child_bottom(Pixels(2.0));

    HStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        params.highpass_res.name(),
        UiData::params,
        params.highpass_res.as_ptr(),
        |params| &params.highpass_res,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
        ParamKnobSize::Regular,
      );
      ParamKnob::new(
        cx,
        params.lowpass_res.name(),
        UiData::params,
        params.lowpass_res.as_ptr(),
        |params| &params.lowpass_res,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
        ParamKnobSize::Regular,
      );
    })
    .size(Auto)
    .child_space(Pixels(4.0))
    .child_bottom(Pixels(2.0));
  })
  .size(Auto)
  .top(Stretch(1.0))
  .bottom(Stretch(1.0))
  .left(Pixels(32.0))
  .border_color("#2d5f4f")
  .border_width(Pixels(2.0))
  .border_top_left_radius(Pixels(12.0))
  .border_top_right_radius(Pixels(0.0))
  .border_bottom_left_radius(Pixels(8.0))
  .border_bottom_right_radius(Pixels(0.0))
}
