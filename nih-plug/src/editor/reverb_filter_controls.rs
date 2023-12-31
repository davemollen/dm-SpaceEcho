#[path="./components/param_knob.rs"]
mod param_knob;
use param_knob::ParamKnob;
use std::sync::Arc;
use nih_plug_vizia::vizia::{
  prelude::{
    Context, LayoutModifiers, StyleModifiers,
    Units::Pixels, LayoutType,
  },
  views::{HStack, ZStack}, handle::Handle
};
use nih_plug::params::Param;
use crate::space_echo_parameters::SpaceEchoParameters;
use super::{UiData, ParamChangeEvent};

pub fn build(cx: &mut Context, params: Arc<SpaceEchoParameters>) -> Handle<HStack> {
  HStack::new(cx, |cx| {
    background(cx);

    ParamKnob::new(
      cx,
      params.reverb.name(),
      UiData::params,
      params.reverb.as_ptr(),
      |params| &params.reverb,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
    ).row_index(0 as usize).col_index(0 as usize);
    ParamKnob::new(
      cx,
      params.decay.name(),
      UiData::params,
      params.decay.as_ptr(),
      |params| &params.decay,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
    ).row_index(0 as usize).col_index(1 as usize);
    
    ParamKnob::new(
      cx,
      params.highpass_freq.name(),
      UiData::params,
      params.highpass_freq.as_ptr(),
      |params| &params.highpass_freq,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
    ).row_index(1 as usize).col_index(0 as usize);
    ParamKnob::new(
      cx,
      params.lowpass_freq.name(),
      UiData::params,
      params.lowpass_freq.as_ptr(),
      |params| &params.lowpass_freq,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
    ).row_index(1 as usize).col_index(1 as usize);

    ParamKnob::new(
      cx,
      params.highpass_res.name(),
      UiData::params,
      params.highpass_res.as_ptr(),
      |params| &params.highpass_res,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
    ).row_index(2 as usize).col_index(0 as usize);
    ParamKnob::new(
      cx,
      params.lowpass_res.name(),
      UiData::params,
      params.lowpass_res.as_ptr(),
      |params| &params.lowpass_res,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
    ).row_index(2 as usize).col_index(1 as usize);
  }).container_style()
}

fn background(cx: &mut Context) {
  ZStack::new(cx, |_| {})
      .col_index(0 as usize)
      .col_span(2 as usize)
      .border_radius_top_left(Pixels(8.0))
      .background_color("#2d5f4f");

  ZStack::new(cx, |_| {})
    .col_index(0 as usize)
    .col_span(2 as usize)
    .row_index(1 as usize)
    .row_span(2 as usize)
    .border_color("#2d5f4f")
    .border_width(Pixels(2.0))
    .border_radius_bottom_left(Pixels(8.0));
}

trait Style {
  fn container_style(self) -> Self;
}

impl<'a> Style for Handle<'a, HStack> {
  fn container_style(self) -> Handle<'a, HStack>{
    self
      .layout_type(LayoutType::Grid)
      .grid_cols(vec![Pixels(152.0); 2])
      .grid_rows(vec![Pixels(184.0); 3])
      .width(Pixels(76.0 * 2.0))
      .top(Pixels(60.0))
      .left(Pixels(24.0))
  }
}