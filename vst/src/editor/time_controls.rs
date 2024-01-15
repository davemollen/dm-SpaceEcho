#[path="./components/param_checkbox.rs"]
mod param_checkbox;
use param_checkbox::ParamCheckbox;
#[path="./components/param_knob.rs"]
mod param_knob;
use param_knob::{ParamKnob, ParamKnobSize};
#[path="./components/param_radio_button.rs"]
mod param_radio_button;
use param_radio_button::ParamRadioButton;
use std::sync::Arc;
use vizia::{
  prelude::{
    Context, LayoutModifiers, StyleModifiers, LensExt,
    Units::Pixels
  },
  views::{HStack, VStack}, 
  view::Handle, layout::Units::{Stretch, Auto}
};
use crate::space_echo_parameters::{SpaceEchoParameters, Params};
use super::{UiData, ParamChangeEvent};

pub fn build(cx: &mut Context, params: Arc<SpaceEchoParameters>) -> Handle<HStack> {
  HStack::new(cx, |cx| {
    ParamKnob::new(
      cx,
      params.input.name,
      UiData::params,
      |params| &params.input,
      |val| ParamChangeEvent::SetInput(val),
      ParamKnobSize::Regular
    )
      .top(Stretch(1.0))
      .bottom(Stretch(1.0))
      .border_color("#2d5f4f")
      .border_width(Pixels(2.0));

    VStack::new(cx, |cx| {
      HStack::new(cx, |cx| {
        ParamCheckbox::new(
          cx,
          params.time_link.name,
          UiData::params,
          |params| &params.time_link,
          |val| ParamChangeEvent::SetTimeLink(val),
        ).width(Pixels(72.0));
        
        ParamKnob::new(
          cx,
          params.time_left.name,
          UiData::params,
          |params| &params.time_left,
          |val| ParamChangeEvent::SetTimeLeft(val),
          ParamKnobSize::Regular
        );
    
        // show when time_link is on
        ParamKnob::new(
          cx,
          params.time_right.name,
          UiData::params,
          |params| &params.time_left,
          |val| ParamChangeEvent::SetTimeLeft(val),
          ParamKnobSize::Regular
        ).toggle_class("hide", UiData::params.map(|p| !p.time_link.get_value()));
    
        // show when time_link is off
        ParamKnob::new(
          cx,
          params.time_right.name,
          UiData::params,
          |params| &params.time_right,
          |val| ParamChangeEvent::SetTimeRight(val),
          ParamKnobSize::Regular
        ).toggle_class("hide", UiData::params.map(|p| p.time_link.get_value()));
        
        ParamKnob::new(
          cx,
          params.feedback.name,
          UiData::params,
          |params| &params.feedback,
          |val| ParamChangeEvent::SetFeedback(val),
          ParamKnobSize::Regular
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
          params.hold.name,
          UiData::params,
          |params| &params.hold,
          |val| ParamChangeEvent::SetHold(val),
        ).width(Pixels(72.0));
        ParamKnob::new(
          cx,
          params.wow_and_flutter.name,
          UiData::params,
          |params| &params.wow_and_flutter,
          |val| ParamChangeEvent::SetWowAndFlutter(val),
          ParamKnobSize::Regular
        );
      })
      .size(Auto)
      .child_space(Pixels(4.0))
      .child_bottom(Pixels(2.0))
      .left(Stretch(1.0));

      HStack::new(cx, |cx| {
        ParamRadioButton::new(
          cx,
          params.time_mode.name,
          UiData::params,
          |params| &params.time_mode,
          |val| ParamChangeEvent::SetTimeMode(val)
        );
        ParamRadioButton::new(
          cx,
          params.channel_mode.name,
          UiData::params,
          |params| &params.channel_mode,
          |val| ParamChangeEvent::SetChannelMode(val)
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
