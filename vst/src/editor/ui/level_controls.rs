#[path="./components/param_checkbox.rs"]
mod param_checkbox;
use param_checkbox::ParamCheckbox;
#[path="./components/param_knob.rs"]
mod param_knob;
use param_knob::ParamKnob;
use std::sync::Arc;
use vizia::{
  prelude::{
    Context, LayoutModifiers, StyleModifiers, TextModifiers,
    Units::{Pixels, Percentage, Stretch}, LayoutType,
  },
  views::{HStack, VStack, Label}, handle::Handle
};
use crate::space_echo_parameters::SpaceEchoParameters;
use super::{UiData, ParamChangeEvent};

pub fn build(cx: &mut Context, params: Arc<SpaceEchoParameters>) -> Handle<HStack> {
  HStack::new(cx, |cx| {
    background(cx);

    ParamKnob::new(
      cx,
      UiData::params,
      &params.output,
      |params| &params.output,
      |val| ParamChangeEvent::SetOutput(val),
    ).row_index(0 as usize).col_index(0 as usize);
    ParamKnob::new(
      cx,
      UiData::params,
      &params.mix,
      |params| &params.mix,
      |val| ParamChangeEvent::SetMix(val),
    ).row_index(0 as usize).col_index(1 as usize);

    ParamKnob::new(
      cx,
      UiData::params,
      &params.duck,
      |params| &params.duck,
      |val| ParamChangeEvent::SetDuck(val),
    ).row_index(1 as usize).col_index(0 as usize);
    
    ParamKnob::new(
      cx,
      UiData::params,
      &params.stereo,
      |params| &params.stereo,
      |val| ParamChangeEvent::SetStereo(val),
    ).row_index(1 as usize).col_index(1 as usize);

    ParamCheckbox::new(
      cx,
      UiData::params,
      &params.limiter,
      |params| &params.limiter,
      |val| ParamChangeEvent::SetLimiter(val),
    ).row_index(0 as usize).col_index(2 as usize);
  }).container_style()
}

fn background(cx: &mut Context) {
  VStack::new(cx, |cx| {
    Label::new(cx, "DM")
      .font_size(32.0)
      .border_color("#2d5f4f")
      .border_radius(Pixels(24.0))
      .border_width(Pixels(4.0))
      .child_top(Pixels(2.0))
      .child_bottom(Pixels(2.0))
      .child_left(Pixels(12.0))
      .child_right(Pixels(12.0));
    Label::new(cx, "SpaceEcho")
      .font_size(22.0);
  })
    .row_index(2 as usize)
    .row_span(2 as usize)
    .col_index(0 as usize)
    .col_span(3 as usize)
    .row_between(Pixels(4.0))
    .child_space(Stretch(1.0))
    .right(Pixels(0.0));
}

trait Style {
  fn container_style(self) -> Self;
}

impl<'a> Style for Handle<'a, HStack> {
  fn container_style(self) -> Handle<'a, HStack>{
    self
      .layout_type(LayoutType::Grid)
      .grid_cols(vec![Pixels(152.0); 3])
      .grid_rows(vec![Pixels(176.0); 4])
      .border_color("#2d5f4f")
      .border_width(Pixels(3.0))
      .border_radius_top_left(Pixels(8.0))
      .width(Percentage(110.0))
      .height(Percentage(110.0))
      .child_space(Pixels(8.0))
  }
}