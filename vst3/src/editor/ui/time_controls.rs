#[path="./components/param_checkbox.rs"]
mod param_checkbox;
use param_checkbox::ParamCheckbox;
#[path="./components/param_knob.rs"]
mod param_knob;
use param_knob::ParamKnob;
#[path="./components/param_radio_button.rs"]
mod param_radio_button;
use param_radio_button::ParamRadioButton;
use std::sync::Arc;
use vizia::{
  prelude::{
    Context, LayoutModifiers, StyleModifiers,
    Units::Pixels, LayoutType,
  },
  views::{HStack, ZStack}, handle::Handle
};
use nih_plug::{params::Param, prelude::Enum};
use crate::space_echo_parameters::{SpaceEchoParameters, TimeMode, ChannelMode};
use super::{UiData, ParamChangeEvent};

pub fn build(cx: &mut Context, params: Arc<SpaceEchoParameters>) -> Handle<HStack> {
  HStack::new(cx, |cx| {
    background(cx);

    ParamKnob::new(
      cx,
      UiData::params,
      params.input.as_ptr(),
      |params| &params.input,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
    )
      .row_index(1 as usize)
      .col_index(0 as usize)
      .border_color("#2d5f4f")
      .border_width(Pixels(2.0));

    ParamCheckbox::new(
      cx,
      UiData::params,
      params.time_link.as_ptr(),
      |params| &params.time_link,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
    ).row_index(0 as usize).col_index(1 as usize);
    ParamKnob::new(
      cx,
      UiData::params,
      params.time_left.as_ptr(),
      |params| &params.time_left,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
    ).row_index(0 as usize).col_index(2 as usize);
    ParamKnob::new(
      cx,
      UiData::params,
      params.time_right.as_ptr(),
      |params| &params.time_right,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
    ).row_index(0 as usize).col_index(3 as usize);
    ParamKnob::new(
      cx,
      UiData::params,
      params.feedback.as_ptr(),
      |params| &params.feedback,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
    ).row_index(0 as usize).col_index(4 as usize);

    ParamCheckbox::new(
      cx,
      UiData::params,
      params.hold.as_ptr(),
      |params| &params.hold,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
    ).row_index(1 as usize).col_index(3 as usize);
    ParamKnob::new(
      cx,
      UiData::params,
      params.wow_and_flutter.as_ptr(),
      |params| &params.wow_and_flutter,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
    ).row_index(1 as usize).col_index(4 as usize);

    ParamRadioButton::new(
      cx,
      UiData::params,
      params.time_mode.as_ptr(),
      |params| &params.time_mode,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
      TimeMode::variants()
    ).row_index(2 as usize).col_index(1 as usize).col_span(2 as usize);
    ParamRadioButton::new(
      cx,
      UiData::params,
      params.channel_mode.as_ptr(),
      |params| &params.channel_mode,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
      ChannelMode::variants()
    ).row_index(2 as usize).col_index(3 as usize).col_span(2 as usize);
  }).container_style()
}

fn background(cx: &mut Context) {
  ZStack::new(cx, |_| {})
      .col_index(1 as usize)
      .col_span(4 as usize)
      .border_radius_top_left(Pixels(8.0))
      .border_radius_top_right(Pixels(8.0))
      .background_color("#2d5f4f");

  ZStack::new(cx, |_| {})
    .col_index(1 as usize)
    .col_span(4 as usize)
    .row_index(1 as usize)
    .row_span(2 as usize)
    .border_color("#2d5f4f")
    .border_width(Pixels(2.0))
    .border_radius_bottom_left(Pixels(8.0))
    .border_radius_bottom_right(Pixels(8.0));
}

trait Style {
  fn container_style(self) -> Self;
}

impl<'a> Style for Handle<'a, HStack> {
  fn container_style(self) -> Handle<'a, HStack>{
    self
      .layout_type(LayoutType::Grid)
      .grid_cols(vec![Pixels(152.0); 5])
      .grid_rows(vec![Pixels(184.0); 3])
      .width(Pixels(76.0 * 5.0))
      .top(Pixels(60.0))
  }
}

