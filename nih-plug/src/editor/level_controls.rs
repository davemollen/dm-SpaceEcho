#[path="./components/param_checkbox.rs"]
mod param_checkbox;
use param_checkbox::ParamCheckbox;
#[path="./components/param_knob.rs"]
mod param_knob;
use param_knob::{ParamKnob, ParamKnobSize};
use std::sync::Arc;
use nih_plug_vizia::vizia::{
  prelude::{
    Context, LayoutModifiers, StyleModifiers, TextModifiers,
    Units::{Pixels, Stretch}
  },
  views::{HStack, VStack, Label},
  view::Handle, layout::Units::Auto, style::FontWeightKeyword
};
use nih_plug::params::Param;
use crate::space_echo_parameters::SpaceEchoParameters;
use super::{UiData, ParamChangeEvent};

pub fn build(cx: &mut Context, params: Arc<SpaceEchoParameters>) -> Handle<VStack> {
  VStack::new(cx, |cx| {
    HStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        params.output.name(),
        UiData::params,
        params.output.as_ptr(),
        |params| &params.output,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
        ParamKnobSize::Regular
      );
      ParamKnob::new(
        cx,
        params.mix.name(),
        UiData::params,
        params.mix.as_ptr(),
        |params| &params.mix,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
        ParamKnobSize::Regular
      );
      ParamCheckbox::new(
        cx,
        params.limiter.name(),
        UiData::params,
        params.limiter.as_ptr(),
        |params| &params.limiter,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
      ).left(Pixels(4.0));
    })
    .size(Auto);

    HStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        params.duck.name(),
        UiData::params,
        params.duck.as_ptr(),
        |params| &params.duck,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
        ParamKnobSize::Regular
      );
      ParamKnob::new(
        cx,
        params.stereo.name(),
        UiData::params,
        params.stereo.as_ptr(),
        |params| &params.stereo,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
        ParamKnobSize::Regular
      );
    })
    .size(Auto)
    .col_between(Pixels(4.0));

    VStack::new(cx, |cx| {
      Label::new(cx, "DM")
        .font_size(24.0)
        .font_weight(FontWeightKeyword::Bold)
        .width(Pixels(64.0))
        .height(Pixels(44.0))
        .child_space(Stretch(1.0))
        .border_color("#2d5f4f")
        .border_width(Pixels(4.0))
        .border_radius(Pixels(16.0));
      Label::new(cx, "SpaceEcho")
        .font_size(22.0)
        .child_space(Stretch(1.0));
    })
    .row_between(Pixels(4.0))
    .child_space(Stretch(1.0));
  })
  .size(Stretch(1.0))
  .child_space(Pixels(4.0))
  .border_width(Pixels(2.0))
  .border_color("#2d5f4f")
  .border_top_left_radius(Pixels(8.0))
}