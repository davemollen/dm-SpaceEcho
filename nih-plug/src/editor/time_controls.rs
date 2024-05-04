#[path = "./components/param_checkbox.rs"]
mod param_checkbox;
use param_checkbox::ParamCheckbox;
#[path = "./components/param_knob.rs"]
mod param_knob;
use param_knob::{ParamKnob, ParamKnobSize};
#[path = "./components/param_radio_button.rs"]
mod param_radio_button;
use super::{ParamChangeEvent, UiData};
use crate::space_echo_parameters::{ChannelMode, SpaceEchoParameters, TimeMode};
use nih_plug::{params::Param, prelude::Enum};
use nih_plug_vizia::vizia::{
  layout::Units::{Auto, Stretch},
  prelude::{Context, LayoutModifiers, LensExt, StyleModifiers, Units::Pixels},
  view::Handle,
  views::{HStack, VStack},
};
use param_radio_button::ParamRadioButton;
use std::sync::Arc;

pub fn build(cx: &mut Context, params: Arc<SpaceEchoParameters>) -> Handle<HStack> {
  HStack::new(cx, |cx| {
    ParamKnob::new(
      cx,
      params.input.name(),
      UiData::params,
      params.input.as_ptr(),
      |params| &params.input,
      |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
      ParamKnobSize::Regular,
    )
    .top(Stretch(1.0))
    .bottom(Stretch(1.0))
    .border_color("#2d5f4f")
    .border_width(Pixels(2.0));

    VStack::new(cx, |cx| {
      HStack::new(cx, |cx| {
        ParamCheckbox::new(
          cx,
          params.time_link.name(),
          UiData::params,
          params.time_link.as_ptr(),
          |params| &params.time_link,
          |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
        )
        .width(Pixels(72.0));

        ParamKnob::new(
          cx,
          params.time_left.name(),
          UiData::params,
          params.time_left.as_ptr(),
          |params| &params.time_left,
          |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          ParamKnobSize::Regular,
        );

        // show when time_link is on
        ParamKnob::new(
          cx,
          params.time_right.name(),
          UiData::params,
          params.time_left.as_ptr(),
          |params| &params.time_left,
          |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          ParamKnobSize::Regular,
        )
        .class("show")
        .toggle_class("hide", UiData::params.map(|p| !p.time_link.value()));

        // show when time_link is off
        ParamKnob::new(
          cx,
          params.time_right.name(),
          UiData::params,
          params.time_right.as_ptr(),
          |params| &params.time_right,
          |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          ParamKnobSize::Regular,
        )
        .class("show")
        .toggle_class("hide", UiData::params.map(|p| p.time_link.value()));

        ParamKnob::new(
          cx,
          params.feedback.name(),
          UiData::params,
          params.feedback.as_ptr(),
          |params| &params.feedback,
          |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          ParamKnobSize::Regular,
        );
      })
      .size(Auto)
      .child_space(Pixels(4.0))
      .child_bottom(Pixels(2.0))
      .background_color("#2d5f4f")
      .border_top_left_radius(Pixels(8.0))
      .border_top_right_radius(Pixels(8.0));

      HStack::new(cx, |cx| {
        ParamCheckbox::new(
          cx,
          params.hold.name(),
          UiData::params,
          params.hold.as_ptr(),
          |params| &params.hold,
          |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
        )
        .width(Pixels(72.0));
        ParamKnob::new(
          cx,
          params.wow_and_flutter.name(),
          UiData::params,
          params.wow_and_flutter.as_ptr(),
          |params| &params.wow_and_flutter,
          |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          ParamKnobSize::Regular,
        );
      })
      .size(Auto)
      .child_space(Pixels(4.0))
      .child_bottom(Pixels(2.0))
      .left(Stretch(1.0));

      HStack::new(cx, |cx| {
        ParamRadioButton::new(
          cx,
          params.time_mode.name(),
          UiData::params,
          params.time_mode.as_ptr(),
          |params| &params.time_mode,
          |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          TimeMode::variants(),
        );
        ParamRadioButton::new(
          cx,
          params.channel_mode.name(),
          UiData::params,
          params.channel_mode.as_ptr(),
          |params| &params.channel_mode,
          |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          ChannelMode::variants(),
        );
      })
      .size(Auto)
      .col_between(Pixels(32.))
      .child_space(Pixels(4.0))
      .child_bottom(Pixels(24.0))
      .left(Stretch(1.0));
    })
    .size(Auto)
    .border_color("#2d5f4f")
    .border_width(Pixels(2.0))
    .border_radius(Pixels(8.0))
    .border_top_left_radius(Pixels(12.0))
    .border_top_right_radius(Pixels(12.0));
  })
  .size(Auto)
  .top(Stretch(1.0))
  .bottom(Stretch(1.0))
}
