#[path="./components/param_knob.rs"]
mod param_knob;
use param_knob::{ParamKnob, ParamKnobSize};
use std::sync::Arc;
use vizia::{
  prelude::{
    Context, LayoutModifiers, StyleModifiers,
    Units::Pixels
  },
  views::{HStack, VStack},
  view::Handle, layout::Units::{Auto, Stretch}
};
use crate::space_echo_parameters::SpaceEchoParameters;
use super::{UiData, ParamChangeEvent};

pub fn build(cx: &mut Context, params: Arc<SpaceEchoParameters>) -> Handle<VStack> {
  VStack::new(cx, |cx| {
    HStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        params.reverb.name,
        UiData::params,
        |params| &params.reverb,
        |val| ParamChangeEvent::SetReverb(val),
        ParamKnobSize::Regular
      );
      ParamKnob::new(
        cx,
        params.decay.name,
        UiData::params,
        |params| &params.decay,
        |val| ParamChangeEvent::SetDecay(val),
        ParamKnobSize::Regular
      );
    })
    .size(Auto)
    .child_space(Pixels(4.0))
    .child_bottom(Pixels(2.0))
    .background_color("#2d5f4f")
    .border_top_left_radius(Pixels(8.0))
    .border_top_right_radius(Pixels(0.0));
    
    HStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        params.highpass_freq.name,
        UiData::params,
        |params| &params.highpass_freq,
        |val| ParamChangeEvent::SetHighpassFreq(val),
        ParamKnobSize::Regular
      );
      ParamKnob::new(
        cx,
        params.lowpass_freq.name,
        UiData::params,
        |params| &params.lowpass_freq,
        |val| ParamChangeEvent::SetLowpassFreq(val),
        ParamKnobSize::Regular
      );
    })
    .size(Auto)
    .child_space(Pixels(4.0))
    .child_bottom(Pixels(2.0));
  
    HStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        params.highpass_res.name,
        UiData::params,
        |params| &params.highpass_res,
        |val| ParamChangeEvent::SetHighpassRes(val),
        ParamKnobSize::Regular
      );
      ParamKnob::new(
        cx,
        params.lowpass_res.name,
        UiData::params,
        |params| &params.lowpass_res,
        |val| ParamChangeEvent::SetLowpassRes(val),
        ParamKnobSize::Regular
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